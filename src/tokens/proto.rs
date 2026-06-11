use super::*;

use serde::de::Error;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Protocol(pub Brc4Value, pub Option<Brc4ActionErr>);

fn bel_20_validate<'de, D>(val: &str) -> Result<Fixed128, D::Error>
where
    D: serde::Deserializer<'de>,
{
    if val.starts_with('+') | val.starts_with('-') {
        return Err(Error::custom("value cannot start from + or -"));
    }
    if val.starts_with('.') | val.ends_with('.') {
        return Err(Error::custom("value cannot start or end with ."));
    }
    if val.starts_with(' ') | val.ends_with(' ') {
        return Err(Error::custom("value cannot contain spaces"));
    }
    match Fixed128::from_str(val) {
        Ok(v) => {
            if v > Fixed128::from(u64::MAX) {
                Err(Error::custom("value is too large"))
            } else {
                Ok(v)
            }
        }
        Err(e) => Err(Error::custom(e)),
    }
}

pub fn bel_20_decimal<'de, D>(deserializer: D) -> Result<Fixed128, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = <&str as serde::Deserialize>::deserialize(deserializer)?;
    bel_20_validate::<D>(val)
}

pub fn bel_20_option_decimal<'de, D>(deserializer: D) -> Result<Option<Fixed128>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = <Option<&str> as serde::Deserialize>::deserialize(deserializer)?;
    val.map(|x| bel_20_validate::<D>(x)).transpose()
}

fn tick_byte_len() -> usize {
    if *BLOCKCHAIN == Blockchain::Wojakcoin {
        8
    } else {
        4
    }
}

fn pad_tick_bytes(bytes: &[u8]) -> Result<OriginalTokenTick, &'static str> {
    let max = tick_byte_len();
    if bytes.is_empty() || bytes.len() > max {
        return Err("invalid token tick");
    }
    let mut tick = [0u8; 8];
    tick[..bytes.len()].copy_from_slice(bytes);
    Ok(OriginalTokenTick(tick))
}

pub fn bel_20_tick<'de, D>(deserializer: D) -> Result<OriginalTokenTick, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = <Cow<str> as serde::Deserialize>::deserialize(deserializer)?;
    pad_tick_bytes(val.as_bytes()).map_err(Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "op")]
#[serde(rename_all = "lowercase")]
pub enum Brc4 {
    Mint {
        #[serde(flatten)]
        proto: MintProto,
    },
    Deploy {
        #[serde(flatten)]
        proto: DeployProto,
    },
    Transfer {
        #[serde(flatten)]
        proto: TransferProto,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MintProtoWrapper {
    #[serde(deserialize_with = "bel_20_tick")]
    pub tick: OriginalTokenTick,
    #[serde(deserialize_with = "bel_20_decimal")]
    pub amt: Fixed128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "p")]
#[serde_as]
pub enum MintProto {
    #[serde(rename = "bel-20")]
    Bel20(MintProtoWrapper),
    #[serde(rename = "drc-20")]
    Drc20(MintProtoWrapper),
    #[serde(rename = "prc-20")]
    Prc20(MintProtoWrapper),
    #[serde(rename = "ltc-20")]
    Ltc20(MintProtoWrapper),
    #[serde(rename = "wjk-20")]
    Wjk20(MintProtoWrapper),
}

impl MintProto {
    pub fn value(&self) -> anyhow::Result<MintProtoWrapper> {
        match self {
            MintProto::Bel20(v) if *BLOCKCHAIN == Blockchain::Bellscoin => Ok(*v),
            MintProto::Drc20(v) if *BLOCKCHAIN == Blockchain::Dogecoin => Ok(*v),
            MintProto::Prc20(v) if *BLOCKCHAIN == Blockchain::Pepecoin => Ok(*v),
            MintProto::Ltc20(v) if *BLOCKCHAIN == Blockchain::Litecoin => Ok(*v),
            MintProto::Wjk20(v) if *BLOCKCHAIN == Blockchain::Wojakcoin => Ok(*v),
            _ => anyhow::bail!("Unsupported type"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DeployProtoWrapper {
    #[serde(deserialize_with = "bel_20_tick")]
    pub tick: OriginalTokenTick,
    #[serde(deserialize_with = "bel_20_decimal")]
    pub max: Fixed128,
    #[serde(default, deserialize_with = "bel_20_option_decimal")]
    pub lim: Option<Fixed128>,
    #[serde(with = ":: serde_with :: As :: < DisplayFromStr >")]
    #[serde(default = "DeployProto::default_dec")]
    pub dec: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "p")]
#[serde_as]
pub enum DeployProto {
    #[serde(rename = "bel-20")]
    Bel20(DeployProtoWrapper),
    #[serde(rename = "drc-20")]
    Drc20(DeployProtoWrapper),
    #[serde(rename = "prc-20")]
    Prc20(DeployProtoWrapper),
    #[serde(rename = "ltc-20")]
    Ltc20(DeployProtoWrapper),
    #[serde(rename = "wjk-20")]
    Wjk20(DeployProtoWrapper),
}

impl DeployProto {
    pub fn value(&self) -> anyhow::Result<DeployProtoWrapper> {
        match self {
            DeployProto::Bel20(v) if *BLOCKCHAIN == Blockchain::Bellscoin => Ok(*v),
            DeployProto::Drc20(v) if *BLOCKCHAIN == Blockchain::Dogecoin => Ok(*v),
            DeployProto::Prc20(v) if *BLOCKCHAIN == Blockchain::Pepecoin => Ok(*v),
            DeployProto::Ltc20(v) if *BLOCKCHAIN == Blockchain::Litecoin => Ok(*v),
            DeployProto::Wjk20(v) if *BLOCKCHAIN == Blockchain::Wojakcoin => Ok(*v),
            _ => anyhow::bail!("Unsupported type"),
        }
    }
}

impl DeployProto {
    pub const DEFAULT_DEC: u8 = 18;
    pub const MAX_DEC: u8 = 18;
    pub fn default_dec() -> u8 {
        Self::DEFAULT_DEC
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "p")]
#[serde_as]
pub enum TransferProto {
    #[serde(rename = "bel-20")]
    Bel20(MintProtoWrapper),
    #[serde(rename = "drc-20")]
    Drc20(MintProtoWrapper),
    #[serde(rename = "prc-20")]
    Prc20(MintProtoWrapper),
    #[serde(rename = "ltc-20")]
    Ltc20(MintProtoWrapper),
    #[serde(rename = "wjk-20")]
    Wjk20(MintProtoWrapper),
}

impl TransferProto {
    pub fn value(&self) -> anyhow::Result<MintProtoWrapper> {
        match self {
            TransferProto::Bel20(v) if *BLOCKCHAIN == Blockchain::Bellscoin => Ok(*v),
            TransferProto::Drc20(v) if *BLOCKCHAIN == Blockchain::Dogecoin => Ok(*v),
            TransferProto::Prc20(v) if *BLOCKCHAIN == Blockchain::Pepecoin => Ok(*v),
            TransferProto::Ltc20(v) if *BLOCKCHAIN == Blockchain::Litecoin => Ok(*v),
            TransferProto::Wjk20(v) if *BLOCKCHAIN == Blockchain::Wojakcoin => Ok(*v),
            _ => anyhow::bail!("Unsupported type"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub enum Brc4Value {
    Mint { tick: OriginalTokenTick, amt: Fixed128 },
    Transfer { tick: OriginalTokenTick, amt: Fixed128 },
    Deploy { tick: OriginalTokenTick, max: Fixed128, lim: Fixed128, dec: u8 },
}

impl TryFrom<&DeployProto> for Brc4Value {
    type Error = anyhow::Error;

    fn try_from(v: &DeployProto) -> Result<Self, Self::Error> {
        let v = v.value()?;
        Ok(Brc4Value::Deploy {
            tick: v.tick,
            max: v.max,
            lim: v.lim.unwrap_or(v.max),
            dec: v.dec,
        })
    }
}

impl TryFrom<&MintProto> for Brc4Value {
    type Error = anyhow::Error;

    fn try_from(v: &MintProto) -> Result<Self, Self::Error> {
        let v = v.value()?;
        Ok(Brc4Value::Mint { tick: v.tick, amt: v.amt })
    }
}

impl TryFrom<&TransferProto> for Brc4Value {
    type Error = anyhow::Error;

    fn try_from(v: &TransferProto) -> Result<Self, Self::Error> {
        let v = v.value()?;
        Ok(Brc4Value::Transfer { tick: v.tick, amt: v.amt })
    }
}

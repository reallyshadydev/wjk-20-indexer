use super::*;

#[derive(Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct TokenBalanceRest {
    pub tick: OriginalTokenTickRest,
    pub balance: Fixed128,
    pub transferable_balance: Fixed128,
    pub transfers: Vec<TokenTransfer>,
    pub transfers_count: u64,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TokenProtoRest {
    pub genesis: InscriptionId,
    pub tick: OriginalTokenTickRest,
    pub max: u64,
    pub lim: u64,
    pub dec: u8,
    pub supply: Fixed128,
    pub mint_count: u64,
    pub transfer_count: u64,
    pub holders: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct AddressOutPoint {
    pub address: FullHash,
    pub outpoint: OutPoint,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Brc4ActionErr {
    NotDeployed,
    AlreadyDeployed,
    ReachDecBound,
    ReachLimBound,
    SupplyMinted,
    InsufficientBalance,
    Transferred,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Brc4ParseErr {
    WrongContentType,
    WrongProtocol,
    DecimalEmpty,
    DecimalOverflow,
    DecimalPlusMinus,
    DecimalDotStartEnd,
    DecimalSpaces,
    InvalidDigit,
    InvalidUtf8,
    Unknown(String),
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Brc4Error {
    Action(Brc4ActionErr),
    Parse(Brc4ParseErr),
}

/// Token tick bytes (4 for BEL/DRC/PRC/LTC, up to 8 for WJK-20 e.g. "wojak").
pub const TICK_STORAGE_LEN: usize = 8;

fn trim_tick_bytes(bytes: &[u8; TICK_STORAGE_LEN]) -> &[u8] {
    let end = bytes.iter().rposition(|b| *b != 0).map(|i| i + 1).unwrap_or(0);
    &bytes[..end]
}

/// Token tick in the original case (same as in the deploy)
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct OriginalTokenTickRest([u8; TICK_STORAGE_LEN]);

impl schemars::JsonSchema for OriginalTokenTickRest {
    fn schema_name() -> Cow<'static, str> {
        "OriginalTokenTick".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "pattern": "^.+$"
        })
    }
}

impl Serialize for OriginalTokenTickRest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = String::from_utf8_lossy(trim_tick_bytes(&self.0));
        serializer.serialize_str(&str)
    }
}

impl<'de> Deserialize<'de> for OriginalTokenTickRest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let b = s.as_bytes();
        if b.is_empty() || b.len() > TICK_STORAGE_LEN {
            return Err(serde::de::Error::custom("Invalid tick length"));
        }
        let mut bytes = [0u8; TICK_STORAGE_LEN];
        bytes[..b.len()].copy_from_slice(b);
        Ok(Self(bytes))
    }
}

impl Display for OriginalTokenTickRest {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(trim_tick_bytes(&self.0)))
    }
}

impl std::fmt::Debug for OriginalTokenTickRest {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl AsRef<[u8]> for OriginalTokenTickRest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<OriginalTokenTick> for OriginalTokenTickRest {
    fn from(value: OriginalTokenTick) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Default, Serialize, Deserialize)]
pub struct OriginalTokenTick(pub [u8; TICK_STORAGE_LEN]);

impl TryFrom<Vec<u8>> for OriginalTokenTick {
    type Error = anyhow::Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        if v.is_empty() || v.len() > TICK_STORAGE_LEN {
            anyhow::bail!("Invalid tick length");
        }
        let mut tick = [0u8; TICK_STORAGE_LEN];
        tick[..v.len()].copy_from_slice(&v);
        Ok(Self(tick))
    }
}

impl From<OriginalTokenTickRest> for OriginalTokenTick {
    fn from(value: OriginalTokenTickRest) -> Self {
        Self(value.0)
    }
}

impl From<[u8; TICK_STORAGE_LEN]> for OriginalTokenTick {
    fn from(v: [u8; TICK_STORAGE_LEN]) -> Self {
        Self(v)
    }
}
impl std::fmt::Debug for OriginalTokenTick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
impl Display for OriginalTokenTick {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(trim_tick_bytes(&self.0)))
    }
}
impl FromStr for OriginalTokenTick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.as_bytes().to_vec())
    }
}
impl From<OriginalTokenTick> for LowerCaseTokenTick {
    fn from(value: OriginalTokenTick) -> Self {
        LowerCaseTokenTick::from(trim_tick_bytes(&value.0))
    }
}

impl From<&OriginalTokenTick> for LowerCaseTokenTick {
    fn from(value: &OriginalTokenTick) -> Self {
        LowerCaseTokenTick::from(trim_tick_bytes(&value.0))
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct InscriptionId {
    pub txid: Txid,
    pub index: u32,
}

impl<'de> Deserialize<'de> for InscriptionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DeserializeFromStr::deserialize(deserializer)?.0)
    }
}

impl Serialize for InscriptionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl Display for InscriptionId {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}i{}", self.txid, self.index)
    }
}

impl From<InscriptionId> for OutPoint {
    fn from(val: InscriptionId) -> Self {
        OutPoint::new(val.txid, val.index)
    }
}

impl From<OutPoint> for InscriptionId {
    fn from(outpoint: OutPoint) -> Self {
        Self {
            txid: outpoint.txid,
            index: outpoint.vout,
        }
    }
}

impl FromStr for InscriptionId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(char) = s.chars().find(|char| !char.is_ascii()) {
            return Err(ParseError::Character(char));
        }

        const TXID_LEN: usize = 64;
        const MIN_LEN: usize = TXID_LEN + 2;

        if s.len() < MIN_LEN {
            return Err(ParseError::Length(s.len()));
        }

        let txid = &s[..TXID_LEN];

        let separator = s.chars().nth(TXID_LEN).ok_or(ParseError::Separator(' '))?;

        if separator != 'i' {
            return Err(ParseError::Separator(separator));
        }

        let vout = &s[TXID_LEN + 1..];

        Ok(Self {
            txid: txid.parse().map_err(ParseError::Txid)?,
            index: vout.parse().map_err(ParseError::Index)?,
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TokenAction {
    /// Deploy new token action.
    Deploy { genesis: InscriptionId, proto: DeployProtoDB, owner: FullHash },
    /// Mint new token action.
    Mint { owner: FullHash, proto: MintProtoWrapper, txid: Txid, vout: u32 },
    /// Transfer token action.
    Transfer {
        location: Location,
        owner: FullHash,
        proto: MintProtoWrapper,
        txid: Txid,
        vout: u32,
    },
    /// Founded move of transfer action.
    Transferred {
        // TokenAction::Transfer location
        transfer_location: Location,
        // if leaked then sender = recipient
        // if burnt them recipient = OP_RETURN_HASH
        recipient: FullHash,
        txid: Txid,
        vout: u32,
    },
}

/// Token transfer
#[derive(Serialize, Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct TokenTransfer {
    pub outpoint: crate::rest::OutPoint,
    pub amount: Fixed128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMeta {
    pub genesis: InscriptionId,
    pub proto: DeployProtoDB,
}

#[derive(Clone, Debug)]
pub struct InscriptionTemplate {
    pub genesis: InscriptionId,
    pub location: Location,
    pub content_type: Option<String>,
    pub owner: FullHash,
    pub value: u64,
    pub content: Option<Vec<u8>>,
    pub leaked: bool,
}

pub(crate) struct DeserializeFromStr<T: FromStr>(pub(crate) T);

impl<'de, T: FromStr> Deserialize<'de> for DeserializeFromStr<T>
where
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(FromStr::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)?))
    }
}

#[derive(Debug)]
pub enum ParseError {
    Character(char),
    Length(usize),
    Separator(char),
    Txid(bellscoin::hashes::hex::Error),
    Index(std::num::ParseIntError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Character(c) => write!(f, "invalid character: '{c}'"),
            Self::Length(len) => write!(f, "invalid length: {len}"),
            Self::Separator(c) => write!(f, "invalid separator: `{c}`"),
            Self::Txid(err) => write!(f, "invalid txid: {err}"),
            Self::Index(err) => write!(f, "invalid index: {err}"),
        }
    }
}

impl std::error::Error for ParseError {}

# WJK-20 Indexer (WojakCoin fork of Nintondo `bel-20-indexer`)

This is a fork of [`Nintondo/bel-20-indexer`](https://github.com/Nintondo/bel-20-indexer)
adapted to index **WJK-20** tokens on **WojakCoin**.

It exposes the same REST API the Wojak Wallet expects, including:

- `GET /address/{address}` and `GET /address/{address}/tokens` — token balances + transfers
- `GET /address/{address}/history`, `GET /tokens`, `GET /token`, `GET /holders`, `GET /status`, …

## What was changed for WojakCoin

All changes keep the multi-chain design intact (Bells/Doge/Pepe/LTC still work);
WojakCoin is added as a new selectable chain.

| File | Change |
| --- | --- |
| `src/blockchain.rs` | Added `Blockchain::Wojakcoin` + `FromStr` (`wojak` / `wojakcoin` / `wjk`). |
| `src/tokens/proto.rs` | Added `wjk-20` variant to `MintProto` / `DeployProto` / `TransferProto` and their `value()` guards. |
| `src/db/structs.rs` | DB → `TransferProto` reconstruction now emits `wjk-20` when the chain is WojakCoin. |
| `src/server/mod.rs` | Maps `(Wojakcoin, mainnet/testnet)` → coin string `wojakcoin` / `wojakcoin-testnet`. |
| `src/main.rs` | `START_HEIGHT` / `JUBILEE_HEIGHT` arms for WojakCoin. |
| `packages/new-blk-parser/src/blockchain/coins.rs` | Added `Wojakcoin` / `WojakcoinTestnet` `Coin` definitions and `CoinType::from_str` entries. |
| `.env.example` | Defaults to `BLOCKCHAIN=wojak`. |

### WojakCoin chain params (base58 version bytes)

Taken from the authoritative `ord-wojakcoin` chainparams
(`crates/rust-wojakcoin-bitcoin/src/blockdata/constants.rs`):

| | mainnet | testnet |
| --- | --- | --- |
| P2PKH (pubkey) | `0x49` (73, `W…`) | `0x71` (113) |
| P2SH (script)  | `0x05` (5)        | `0xc4` (196) |

WojakCoin is pre-segwit, so the `bech32` hrp is unused (set to `wjk`).

## Running

> Requires a Rust toolchain (`cargo`) and a synced WojakCoin node with `-txindex`
> and RPC enabled. Neither is bundled here.

1. Copy `.env.example` to `.env` and fill in the WojakCoin RPC details:

```env
RPC_URL=http://127.0.0.1:20760
RPC_USER=<rpcuser>
RPC_PASS=<rpcpassword>
BLOCKCHAIN=wojak
# NETWORK unset = mainnet
# SERVER_BIND_URL=0.0.0.0:8000
```

2. Build & run (RPC mode — no blk files needed):

```bash
cargo r -r
```

3. The REST API listens on `SERVER_BIND_URL` (default `0.0.0.0:8000`).
   Point the wallet's token-balance base URL at this server.

### Faster initial sync (optional)

Set `BLK_DIR` to the node's `blocks/` dir and `INDEX_DIR` to an rsync'd copy of
the LevelDB block index (see the upstream README). Note: blk-file framing magic
is read from the files themselves, so no extra per-coin magic constant is needed;
if WojakCoin's blk magic differs and parsing fails, fall back to RPC mode.

## Not yet validated in this environment

The code changes above are complete and internally consistent, but have **not**
been compiled or run here (no Rust toolchain / WojakCoin node available). Before
production use:

- `cargo build -r` to confirm it compiles.
- Index against a WojakCoin node and verify `/address/{W…}/tokens` returns the
  expected WJK-20 balances for a known minter.
- Confirm `START_HEIGHT` (currently `1`) matches the first WJK-20 deploy height;
  raise it to skip empty early blocks if desired.

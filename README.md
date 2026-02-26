# Hyperliquid API — Examples

Clone-and-run examples for the [Hyperliquid API](https://hyperliquidapi.com) builder API.
Trade perps, spot, and HIP-3 markets on HyperLiquid through a single JSON-RPC endpoint.
Your private key never leaves your machine — you sign locally, the service forwards to HyperLiquid.

## Prerequisites

Export your private key (hex, with or without `0x` prefix):

```bash
export PRIVATE_KEY="0xYOUR_HEX_PRIVATE_KEY"
```

---

## Python

```bash
cd python
pip install -r requirements.txt
```

Run any example:

```bash
python3 approve.py          # one-time builder fee approval (run first)
python3 check_status.py     # check approval status
python3 list_markets.py     # list all perps, spot, and HIP-3 markets
python3 open_orders.py      # view open orders with cancel actions
python3 order_status.py 123 # check what happened to order by OID
python3 place_order.py      # place BTC perp IOC buy (~$11)
python3 cancel_order.py     # place resting order, then cancel it
python3 modify_order.py     # place resting order, then modify its price
python3 roundtrip.py        # IOC buy then sell (full trade cycle)
python3 hip3_order.py       # HIP-3 market order (xyz:SILVER)
python3 revoke.py           # revoke builder fee approval
```

---

## TypeScript

```bash
cd typescript
npm install
```

Run any example:

```bash
npx ts-node src/approve.ts       # one-time builder fee approval (run first)
npx ts-node src/checkStatus.ts   # check approval status
npx ts-node src/listMarkets.ts   # list all markets
npx ts-node src/openOrders.ts    # view open orders with cancel actions
npx ts-node src/orderStatus.ts 123  # check what happened to order by OID
npx ts-node src/placeOrder.ts    # place BTC perp IOC buy (~$11)
npx ts-node src/cancelOrder.ts   # place resting order, then cancel
npx ts-node src/modifyOrder.ts   # place resting order, then modify
npx ts-node src/roundtrip.ts     # IOC buy then sell (full trade cycle)
npx ts-node src/hip3Order.ts     # HIP-3 market order (xyz:SILVER)
npx ts-node src/revoke.ts        # revoke builder fee approval
```

---

## Rust

```bash
cd rust
```

Run any example:

```bash
cargo run --example approve         # one-time builder fee approval (run first)
cargo run --example check_status    # check approval status
cargo run --example list_markets    # list all markets
cargo run --example open_orders     # view open orders with cancel actions
cargo run --example order_status -- 123  # check what happened to order by OID
cargo run --example place_order     # place BTC perp IOC buy (~$11)
cargo run --example cancel_order    # place resting order, then cancel
cargo run --example roundtrip       # IOC buy then sell (full trade cycle)
cargo run --example revoke          # revoke builder fee approval
```

---

## How It Works

Every example follows the same pattern:

1. **Build** — call `hl_build*` to get a hash (the service computes the EIP-712 hash, injects builder fee)
2. **Sign** — sign the 32-byte hash locally with your private key
3. **Send** — call `hl_send*` with the signature (the service forwards to HyperLiquid)

All coin references use names (`"BTC"`, `"ETH"`, `"xyz:SILVER"`) — the service resolves them to numeric indices.

## API Docs

Full API reference, error codes, and fees: **https://hyperliquidapi.com**

Wallet-based approval (no code needed): **https://hyperliquidapi.com/approve**

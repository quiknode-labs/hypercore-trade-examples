"""Place a perp limit order (BTC, IOC, ~$10 notional)."""

import json
from client import exchange, sign_hash, get_mid

COIN = "BTC"

mid = get_mid(COIN)
if mid == 0:
    print(f"Could not fetch {COIN} mid price")
    raise SystemExit(1)

sz = round(11.0 / mid, 5)
buy_px = int(mid * 1.03)

print(f"{COIN} mid: ${mid:,.2f}")
print(f"BUY {sz} @ {buy_px} (IOC, ~${sz * mid:.2f} notional)")

res = exchange({
    "action": {
        "type": "order",
        "orders": [{
            "asset": COIN,
            "side": "buy",
            "price": str(buy_px),
            "size": str(sz),
            "tif": "ioc",
        }],
    },
})
sig = sign_hash(res["hash"])

result = exchange({
    "action": res["action"],
    "nonce": res["nonce"],
    "signature": sig,
})

print(json.dumps(result["exchangeResponse"], indent=2))

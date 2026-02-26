"""Place an IOC order on a HIP-3 market by name (e.g. xyz:SILVER)."""

import json
from client import exchange, sign_hash, get_hip3_mid

COIN = "xyz:SILVER"

mid = get_hip3_mid(COIN)
if mid == 0:
    print(f"Could not fetch {COIN} mid price, using fallback")
    mid = 78.0

sz = round(11.0 / mid, 2)
buy_px = round(mid * 1.03, 2)

print(f"{COIN} mid: ${mid:,.2f}")
print(f"BUY {sz} @ {buy_px} (IOC, ~${sz * mid:.2f} notional)\n")

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

"""Place a market order — no price needed, worker computes it automatically."""

import json
from client import exchange, sign_hash

COIN = "BTC"
SIZE = "0.00011"

print(f"Market BUY {SIZE} {COIN}\n")

res = exchange({
    "action": {
        "type": "order",
        "orders": [{
            "asset": COIN,
            "side": "buy",
            "size": SIZE,
            "tif": "market",
        }],
    },
})

computed_price = res["action"]["orders"][0]["p"]
print(f"Computed price (mid + 3% slippage): {computed_price}")
print(f"Builder fee: {res['builderFee']}")

sig = sign_hash(res["hash"])

result = exchange({
    "action": res["action"],
    "nonce": res["nonce"],
    "signature": sig,
})

print(json.dumps(result["exchangeResponse"], indent=2))

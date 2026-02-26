"""Close a position — worker queries your position and builds the counter-order."""

import json
from client import exchange, sign_hash, address

COIN = "HYPE"

print(f"Closing {COIN} position for {address}\n")

res = exchange({
    "action": {
        "type": "closePosition",
        "asset": COIN,
        "user": address,
    },
})

ctx = res.get("closePositionContext", {})
print(f"Position: {ctx.get('positionSize')} {ctx.get('positionSide')}")
print(f"Close: {ctx.get('closeSide')} {ctx.get('closeSize')} @ {ctx.get('slippedPrice')}")

sig = sign_hash(res["hash"])

result = exchange({
    "action": res["action"],
    "nonce": res["nonce"],
    "signature": sig,
})

print(json.dumps(result["exchangeResponse"], indent=2))
print("\nPosition closed.")

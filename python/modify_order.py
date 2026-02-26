"""Place a resting order (GTC, 3% below mid), then modify its price to 4% below."""

import json
from client import exchange, sign_hash, get_mid

COIN = "BTC"

mid = get_mid(COIN)
if mid == 0:
    print(f"Could not fetch {COIN} mid price")
    raise SystemExit(1)

sz = round(11.0 / mid, 5)
rest_px = int(mid * 0.97)

print(f"{COIN} mid: ${mid:,.2f}")
print(f"Placing resting BUY {sz} @ {rest_px} (GTC, 3% below mid)\n")

res = exchange({
    "action": {
        "type": "order",
        "orders": [{
            "asset": COIN,
            "side": "buy",
            "price": str(rest_px),
            "size": str(sz),
            "tif": "gtc",
        }],
    },
})
sig = sign_hash(res["hash"])

result = exchange({
    "action": res["action"],
    "nonce": res["nonce"],
    "signature": sig,
})

exchange_resp = result["exchangeResponse"]
statuses = exchange_resp.get("response", {}).get("data", {}).get("statuses", [])

oid = None
for s in statuses:
    if isinstance(s, dict) and "resting" in s:
        oid = s["resting"].get("oid")
        break

if oid is None:
    print("Could not extract OID from resting order")
    print(json.dumps(exchange_resp, indent=2))
    raise SystemExit(1)

new_px = int(mid * 0.96)
print(f"Order resting (OID: {oid})")
print(f"Modifying price: {rest_px} -> {new_px}\n")

modify_action = {
    "type": "batchModify",
    "modifies": [{
        "oid": oid,
        "order": {
            "asset": COIN,
            "side": "buy",
            "price": str(new_px),
            "size": str(sz),
            "tif": "gtc",
        },
    }],
}

res = exchange({"action": modify_action})
sig = sign_hash(res["hash"])

result = exchange({
    "action": modify_action,
    "nonce": res["nonce"],
    "signature": sig,
})

print(json.dumps(result["exchangeResponse"], indent=2))
print("\nOrder modified.")

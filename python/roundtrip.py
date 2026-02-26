"""IOC buy then sell on BTC perp — full trade cycle (~$11 notional)."""

import json
import time
from client import exchange, sign_hash, get_mid

COIN = "BTC"

mid = get_mid(COIN)
if mid == 0:
    print(f"Could not fetch {COIN} mid price")
    raise SystemExit(1)

sz = round(11.0 / mid, 5)
print(f"{COIN} mid: ${mid:,.2f}")
print(f"Trade size: {sz} {COIN} (~${sz * mid:.2f})\n")


def send_order(side, px):
    res = exchange({
        "action": {
            "type": "order",
            "orders": [{
                "asset": COIN,
                "side": side,
                "price": str(px),
                "size": str(sz),
                "tif": "ioc",
            }],
        },
    })
    sig = sign_hash(res["hash"])
    return exchange({
        "action": res["action"],
        "nonce": res["nonce"],
        "signature": sig,
    })


buy_px = int(mid * 1.03)
print(f"BUY {sz} @ {buy_px} (IOC)")
buy_result = send_order("buy", buy_px)
buy_resp = buy_result["exchangeResponse"]
statuses = buy_resp.get("response", {}).get("data", {}).get("statuses", [])
for s in statuses:
    if isinstance(s, dict) and "error" in s:
        print(f"Buy error: {s['error']}")
        raise SystemExit(1)

print(f"Buy filled: {json.dumps(buy_resp, indent=2)}\n")
time.sleep(1)

sell_px = int(mid * 0.97)
print(f"SELL {sz} @ {sell_px} (IOC)")
sell_result = send_order("sell", sell_px)
sell_resp = sell_result["exchangeResponse"]
statuses = sell_resp.get("response", {}).get("data", {}).get("statuses", [])
for s in statuses:
    if isinstance(s, dict) and "error" in s:
        print(f"Sell error: {s['error']}")
        raise SystemExit(1)

print(f"Sell filled: {json.dumps(sell_resp, indent=2)}")
print("\nRound-trip complete. Position should be flat.")

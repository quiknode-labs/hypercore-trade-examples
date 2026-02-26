"""View open orders with enriched asset info and pre-built cancel actions."""

import json
from client import post_endpoint, address

result = post_endpoint("/openOrders", {"user": address})

print(f"Open orders: {result['count']}")
for order in result["orders"]:
    side = "BUY" if order["side"] == "B" else "SELL"
    spot = " [SPOT]" if order.get("isSpot") else ""
    print(f"  {order['name']}{spot} {side} {order['sz']} @ {order['limitPx']} (OID: {order['oid']})")

if result["count"] > 0:
    print(f"\nCancel actions by asset:")
    for name, action in result["cancelActions"]["byAsset"].items():
        count = len(action["cancels"])
        print(f"  {name}: {count} order(s) — pass as action to POST /exchange")

    print(f"\nTo cancel ALL orders, pass cancelActions.all as the action to POST /exchange:")
    print(json.dumps(result["cancelActions"]["all"], indent=2))

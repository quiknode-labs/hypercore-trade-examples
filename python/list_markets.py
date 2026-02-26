"""List all available markets (perps, spot, HIP-3)."""

from client import get_markets

data = get_markets()

perps = data.get("perps", [])
spot = data.get("spot", [])
hip3_raw = data.get("hip3", {})

hip3_all = []
for dex, markets in hip3_raw.items():
    for m in markets:
        hip3_all.append({**m, "dex": dex})

print(f"Perps: {len(perps)}  |  Spot: {len(spot)}  |  HIP-3: {len(hip3_all)}\n")

for label, group in [("Perps", perps), ("Spot", spot)]:
    if group:
        print(f"--- {label} ---")
        for m in group[:10]:
            print(f"  {m['name']:16s}  index={m['index']}  szDecimals={m['szDecimals']}")
        if len(group) > 10:
            print(f"  ... and {len(group) - 10} more")
        print()

if hip3_all:
    print("--- HIP-3 ---")
    for m in hip3_all[:10]:
        display = f"{m['dex']}:{m['name']}"
        print(f"  {display:16s}  index={m['index']}  szDecimals={m['szDecimals']}")
    if len(hip3_all) > 10:
        print(f"  ... and {len(hip3_all) - 10} more")
    print()

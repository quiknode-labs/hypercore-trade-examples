"""REST client for Hyperliquid API (via QuickNode builder API).

No SDK required -- just requests + eth_account.
"""

import json
import os
import sys

import requests
from eth_account import Account

API_URL = "https://send.hyperliquidapi.com"
HL_INFO_URL = "https://api.hyperliquid.xyz/info"

_pk = os.environ.get("PRIVATE_KEY")
if not _pk:
    print("Set PRIVATE_KEY environment variable (hex, with or without 0x)")
    sys.exit(1)

wallet = Account.from_key(_pk)
address = wallet.address
print(f"Wallet: {address}")


def exchange(body):
    """POST /exchange -- build (no signature) or send (with signature)."""
    r = requests.post(f"{API_URL}/exchange", json=body)
    data = r.json()
    if data.get("error"):
        print(f"\nError ({r.status_code}):")
        print(f"  error:    {data.get('error')}")
        print(f"  message:  {data.get('message')}")
        guidance = data.get("guidance")
        if guidance:
            print(f"  guidance: {guidance}")
        sys.exit(1)
    return data


def get_approval(user):
    """GET /approval?user=<addr> -- check builder fee approval status."""
    r = requests.get(f"{API_URL}/approval", params={"user": user})
    return r.json()


def get_markets():
    """GET /markets -- list all available markets."""
    r = requests.get(f"{API_URL}/markets")
    return r.json()


def post_endpoint(path, body):
    """POST to a utility endpoint (e.g. /openOrders, /orderStatus, /preflight)."""
    r = requests.post(f"{API_URL}{path}", json=body)
    return r.json()


def sign_hash(hash_hex):
    h = bytes.fromhex(hash_hex.removeprefix("0x"))
    s = wallet.unsafe_sign_hash(h)
    return {"r": f"0x{s.r:064x}", "s": f"0x{s.s:064x}", "v": s.v}


def get_mid(coin):
    """Get the current mid price for a coin from Hyperliquid."""
    r = requests.post(HL_INFO_URL, json={"type": "allMids"})
    return float(r.json().get(coin, 0))


def get_hip3_mid(coin):
    """Get mid price for a HIP-3 market (requires dex parameter)."""
    dex = coin.split(":")[0]
    r = requests.post(HL_INFO_URL, json={"type": "allMids", "dex": dex})
    return float(r.json().get(coin, 0))

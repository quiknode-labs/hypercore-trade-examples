"""Thin RPC client for Hyperliquid API builder API."""

import json
import os
import sys

import requests
from eth_account import Account

ENDPOINT = "https://send.hyperliquidapi.com"

_pk = os.environ.get("PRIVATE_KEY")
if not _pk:
    print("Set PRIVATE_KEY environment variable (hex, with or without 0x)")
    sys.exit(1)

wallet = Account.from_key(_pk)
print(f"Wallet: {wallet.address}")

_req_id = 0


def rpc(method, params=None):
    global _req_id
    _req_id += 1
    r = requests.post(ENDPOINT, json={
        "jsonrpc": "2.0",
        "method": method,
        "params": params or {},
        "id": _req_id,
    })
    data = r.json()
    if data.get("error"):
        err = data["error"]
        print(f"\nRPC error ({method}):")
        print(f"  code:     {err.get('code')}")
        print(f"  message:  {err.get('message')}")
        guidance = err.get("data", {}).get("guidance")
        if guidance:
            print(f"  guidance: {guidance}")
        sys.exit(1)
    return data


def sign_hash(hash_hex):
    h = bytes.fromhex(hash_hex.removeprefix("0x"))
    s = wallet.unsafe_sign_hash(h)
    return {"r": f"0x{s.r:064x}", "s": f"0x{s.s:064x}", "v": s.v}


def get_mid(coin):
    """Get the current mid price for a coin from Hyperliquid."""
    r = requests.post("https://api.hyperliquid.xyz/info", json={"type": "allMids"})
    return float(r.json().get(coin, 0))

"""Approve builder fee (one-time setup)."""

from client import exchange, sign_hash

MAX_FEE = "1%"

res = exchange({
    "action": {"type": "approveBuilderFee", "maxFeeRate": MAX_FEE},
})
sig = sign_hash(res["hash"])

exchange({
    "action": {"type": "approveBuilderFee", "maxFeeRate": MAX_FEE},
    "nonce": res["nonce"],
    "signature": sig,
})

print("Builder fee approved.")

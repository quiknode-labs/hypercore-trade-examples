"""Revoke builder fee approval."""

from client import exchange, sign_hash

res = exchange({
    "action": {"type": "approveBuilderFee", "maxFeeRate": "0%"},
})
sig = sign_hash(res["hash"])

exchange({
    "action": {"type": "approveBuilderFee", "maxFeeRate": "0%"},
    "nonce": res["nonce"],
    "signature": sig,
})

print("Builder fee revoked.")

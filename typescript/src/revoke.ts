import { exchange, signHash } from "./client";

async function main() {
  const res = await exchange({
    action: { type: "approveBuilderFee", maxFeeRate: "0%" },
  });
  const sig = await signHash(res.hash);

  await exchange({
    action: { type: "approveBuilderFee", maxFeeRate: "0%" },
    nonce: res.nonce,
    signature: sig,
  });

  console.log("Builder fee revoked.");
}

main();

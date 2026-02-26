import { exchange, signHash } from "./client";

const MAX_FEE = "1%";

async function main() {
  const res = await exchange({
    action: { type: "approveBuilderFee", maxFeeRate: MAX_FEE },
  });
  const sig = await signHash(res.hash);

  await exchange({
    action: { type: "approveBuilderFee", maxFeeRate: MAX_FEE },
    nonce: res.nonce,
    signature: sig,
  });

  console.log("Builder fee approved.");
}

main();

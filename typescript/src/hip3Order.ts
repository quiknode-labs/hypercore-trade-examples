import { exchange, signHash, getHip3Mid } from "./client";

const COIN = "xyz:SILVER";

async function main() {
  let mid = await getHip3Mid(COIN);
  if (mid === 0) {
    console.log(`Could not fetch ${COIN} mid price, using fallback`);
    mid = 78.0;
  }

  const sz = (11.0 / mid).toFixed(2);
  const buyPx = (mid * 1.03).toFixed(2);

  console.log(`${COIN} mid: $${mid.toFixed(2)}`);
  console.log(`BUY ${sz} @ ${buyPx} (IOC, ~$${(parseFloat(sz) * mid).toFixed(2)} notional)\n`);

  const res = await exchange({
    action: {
      type: "order",
      orders: [{ asset: COIN, side: "buy", price: buyPx, size: sz, tif: "ioc" }],
    },
  });
  const sig = await signHash(res.hash);

  const result = await exchange({
    action: res.action,
    nonce: res.nonce,
    signature: sig,
  });

  console.log(JSON.stringify(result.exchangeResponse, null, 2));
}

main();

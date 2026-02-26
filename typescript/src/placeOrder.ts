import { exchange, signHash, getMid } from "./client";

const COIN = "BTC";

async function main() {
  const mid = await getMid(COIN);
  if (mid === 0) {
    console.error(`Could not fetch ${COIN} mid price`);
    process.exit(1);
  }

  const sz = (11.0 / mid).toFixed(5);
  const buyPx = Math.floor(mid * 1.03).toString();

  console.log(`${COIN} mid: $${mid.toLocaleString()}`);
  console.log(`BUY ${sz} @ ${buyPx} (IOC, ~$${(parseFloat(sz) * mid).toFixed(2)} notional)`);

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

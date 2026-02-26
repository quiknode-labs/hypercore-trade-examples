import { exchange, signHash, getMid } from "./client";

const COIN = "BTC";

async function sendOrder(side: string, px: string, sz: string) {
  const res = await exchange({
    action: {
      type: "order",
      orders: [{ asset: COIN, side, price: px, size: sz, tif: "ioc" }],
    },
  });
  const sig = await signHash(res.hash);

  return exchange({
    action: res.action,
    nonce: res.nonce,
    signature: sig,
  });
}

function checkStatuses(resp: any, label: string): boolean {
  const statuses = resp?.response?.data?.statuses || [];
  for (const s of statuses) {
    if (s.error) {
      console.error(`${label} error: ${s.error}`);
      return false;
    }
  }
  return true;
}

async function main() {
  const mid = await getMid(COIN);
  if (mid === 0) {
    console.error(`Could not fetch ${COIN} mid price`);
    process.exit(1);
  }

  const sz = (11.0 / mid).toFixed(5);
  console.log(`${COIN} mid: $${mid.toLocaleString()}`);
  console.log(`Trade size: ${sz} ${COIN} (~$${(parseFloat(sz) * mid).toFixed(2)})\n`);

  const buyPx = Math.floor(mid * 1.03).toString();
  console.log(`BUY ${sz} @ ${buyPx} (IOC)`);
  const buyResult = await sendOrder("buy", buyPx, sz);
  const buyResp = buyResult.exchangeResponse;
  if (!checkStatuses(buyResp, "BUY")) {
    process.exit(1);
  }
  console.log(`Buy filled: ${JSON.stringify(buyResp, null, 2)}\n`);

  await new Promise((r) => setTimeout(r, 1000));

  const sellPx = Math.floor(mid * 0.97).toString();
  console.log(`SELL ${sz} @ ${sellPx} (IOC)`);
  const sellResult = await sendOrder("sell", sellPx, sz);
  const sellResp = sellResult.exchangeResponse;
  if (!checkStatuses(sellResp, "SELL")) {
    process.exit(1);
  }
  console.log(`Sell filled: ${JSON.stringify(sellResp, null, 2)}`);
  console.log("\nRound-trip complete. Position should be flat.");
}

main();

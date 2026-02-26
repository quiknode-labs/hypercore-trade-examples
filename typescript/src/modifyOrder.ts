import { exchange, signHash, getMid } from "./client";

const COIN = "BTC";

async function main() {
  const mid = await getMid(COIN);
  if (mid === 0) {
    console.error(`Could not fetch ${COIN} mid price`);
    process.exit(1);
  }

  const sz = (11.0 / mid).toFixed(5);
  const restPx = Math.floor(mid * 0.97).toString();

  console.log(`${COIN} mid: $${mid.toLocaleString()}`);
  console.log(`Placing resting BUY ${sz} @ ${restPx} (GTC, 3% below mid)\n`);

  let res = await exchange({
    action: {
      type: "order",
      orders: [{ asset: COIN, side: "buy", price: restPx, size: sz, tif: "gtc" }],
    },
  });
  let sig = await signHash(res.hash);

  const result = await exchange({
    action: res.action,
    nonce: res.nonce,
    signature: sig,
  });

  const exchangeResp = result.exchangeResponse;
  const statuses = exchangeResp?.response?.data?.statuses || [];

  let oid: number | null = null;
  for (const s of statuses) {
    if (s.resting) {
      oid = s.resting.oid;
      break;
    }
  }

  if (oid === null) {
    console.error("Could not extract OID from resting order");
    console.log(JSON.stringify(exchangeResp, null, 2));
    process.exit(1);
  }

  const newPx = Math.floor(mid * 0.96).toString();
  console.log(`Order resting (OID: ${oid})`);
  console.log(`Modifying price: ${restPx} -> ${newPx}\n`);

  const modifyAction = {
    type: "batchModify",
    modifies: [{
      oid,
      order: { asset: COIN, side: "buy", price: newPx, size: sz, tif: "gtc" },
    }],
  };

  res = await exchange({ action: modifyAction });
  sig = await signHash(res.hash);

  const modifyResult = await exchange({
    action: modifyAction,
    nonce: res.nonce,
    signature: sig,
  });

  console.log(JSON.stringify(modifyResult.exchangeResponse, null, 2));
  console.log("\nOrder modified.");
}

main();

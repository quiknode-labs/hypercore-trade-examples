import { postEndpoint, address } from "./client";

async function main() {
  const oid = parseInt(process.argv[2] || "0", 10);
  if (oid === 0) {
    console.log("Usage: npx ts-node src/orderStatus.ts <oid>");
    console.log("Get OIDs from: npx ts-node src/openOrders.ts");
    process.exit(1);
  }

  const result = await postEndpoint("/orderStatus", { user: address, oid });

  if (result.status === "unknownOid") {
    console.log(`Order ${oid}: not found`);
    console.log(`  ${result.explanation}`);
    process.exit(0);
  }

  const spot = result.isSpot ? " [SPOT]" : "";
  console.log(`Order ${oid} on ${result.name}${spot}: ${result.status}`);
  console.log(`  ${result.explanation}`);
  if (result.order) {
    console.log(`  Details: ${JSON.stringify(result.order, null, 2)}`);
  }
}

main();

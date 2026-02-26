import { getApproval, address } from "./client";

async function main() {
  const res = await getApproval(address);
  console.log(JSON.stringify(res, null, 2));
}

main();

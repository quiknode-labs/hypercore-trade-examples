import { Wallet } from "ethers";

const ENDPOINT = "https://send.hyperliquidapi.com";

const pk = process.env.PRIVATE_KEY;
if (!pk) {
  console.error("Set PRIVATE_KEY environment variable (hex, with or without 0x)");
  process.exit(1);
}

export const wallet = new Wallet(pk);
console.log(`Wallet: ${wallet.address}`);

let reqId = 0;

export async function rpc(method: string, params: Record<string, any> = {}): Promise<any> {
  reqId++;
  const res = await fetch(ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", method, params, id: reqId }),
  });
  const data: any = await res.json();
  if (data.error) {
    console.error(`\nRPC error (${method}):`);
    console.error(`  code:     ${data.error.code}`);
    console.error(`  message:  ${data.error.message}`);
    if (data.error.data?.guidance) {
      console.error(`  guidance: ${data.error.data.guidance}`);
    }
    process.exit(1);
  }
  return data;
}

export async function signHash(hashHex: string): Promise<{ r: string; s: string; v: number }> {
  const sig = wallet.signingKey.sign(hashHex);
  return { r: sig.r, s: sig.s, v: sig.v };
}

export async function getMid(coin: string): Promise<number> {
  const res = await fetch("https://api.hyperliquid.xyz/info", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ type: "allMids" }),
  });
  const data: any = await res.json();
  return parseFloat(data[coin] || "0");
}

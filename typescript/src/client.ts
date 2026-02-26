import { Wallet } from "ethers";

const API_URL = "https://send.hyperliquidapi.com";
const HL_INFO_URL = "https://api.hyperliquid.xyz/info";

const pk = process.env.PRIVATE_KEY;
if (!pk) {
  console.error("Set PRIVATE_KEY environment variable (hex, with or without 0x)");
  process.exit(1);
}

export const wallet = new Wallet(pk);
export const address = wallet.address;
console.log(`Wallet: ${address}`);

export async function exchange(body: Record<string, any>): Promise<any> {
  const res = await fetch(`${API_URL}/exchange`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  const data: any = await res.json();
  if (data.error) {
    console.error(`\nError (${res.status}):`);
    console.error(`  error:    ${data.error}`);
    console.error(`  message:  ${data.message}`);
    if (data.guidance) {
      console.error(`  guidance: ${data.guidance}`);
    }
    process.exit(1);
  }
  return data;
}

export async function getApproval(user: string): Promise<any> {
  const res = await fetch(`${API_URL}/approval?user=${user}`);
  return res.json();
}

export async function getMarkets(): Promise<any> {
  const res = await fetch(`${API_URL}/markets`);
  return res.json();
}

export async function postEndpoint(path: string, body: Record<string, any>): Promise<any> {
  const res = await fetch(`${API_URL}${path}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  return res.json();
}

export async function signHash(hashHex: string): Promise<{ r: string; s: string; v: number }> {
  const sig = wallet.signingKey.sign(hashHex);
  return { r: sig.r, s: sig.s, v: sig.v };
}

export async function getMid(coin: string): Promise<number> {
  const res = await fetch(HL_INFO_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ type: "allMids" }),
  });
  const data: any = await res.json();
  return parseFloat(data[coin] || "0");
}

export async function getHip3Mid(coin: string): Promise<number> {
  const dex = coin.split(":")[0];
  const res = await fetch(HL_INFO_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ type: "allMids", dex }),
  });
  const data: any = await res.json();
  return parseFloat(data[coin] || "0");
}

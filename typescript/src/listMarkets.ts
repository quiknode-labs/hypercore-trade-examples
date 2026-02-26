import { getMarkets } from "./client";

interface Market {
  name: string;
  index: number;
  szDecimals: number;
}

async function main() {
  const data: { perps: Market[]; spot: Market[]; hip3: Record<string, Market[]> } = await getMarkets();

  const { perps, spot, hip3 } = data;

  const hip3All: (Market & { dex: string })[] = [];
  for (const [dex, markets] of Object.entries(hip3)) {
    for (const m of markets) {
      hip3All.push({ ...m, dex });
    }
  }

  console.log(`Perps: ${perps.length}  |  Spot: ${spot.length}  |  HIP-3: ${hip3All.length}\n`);

  for (const [label, group] of [["Perps", perps], ["Spot", spot]] as [string, Market[]][]) {
    if (group.length > 0) {
      console.log(`--- ${label} ---`);
      for (const m of group.slice(0, 10)) {
        console.log(`  ${m.name.padEnd(16)}  index=${m.index}  szDecimals=${m.szDecimals}`);
      }
      if (group.length > 10) {
        console.log(`  ... and ${group.length - 10} more`);
      }
      console.log();
    }
  }

  if (hip3All.length > 0) {
    console.log("--- HIP-3 ---");
    for (const m of hip3All.slice(0, 10)) {
      const display = `${m.dex}:${m.name}`;
      console.log(`  ${display.padEnd(16)}  index=${m.index}  szDecimals=${m.szDecimals}`);
    }
    if (hip3All.length > 10) {
      console.log(`  ... and ${hip3All.length - 10} more`);
    }
    console.log();
  }
}

main();

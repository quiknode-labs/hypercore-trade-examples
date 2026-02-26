use hyperliquid_api_examples::Client;
use serde_json::Value;

fn print_group(label: &str, markets: &[Value]) {
    if markets.is_empty() {
        return;
    }
    println!("--- {label} ---");
    for m in markets.iter().take(10) {
        let name = m["name"].as_str().unwrap_or("?");
        let index = &m["index"];
        let sz = &m["szDecimals"];
        println!("  {name:<16}  index={index}  szDecimals={sz}");
    }
    if markets.len() > 10 {
        println!("  ... and {} more", markets.len() - 10);
    }
    println!();
}

#[tokio::main]
async fn main() {
    let client = Client::from_env();
    let res = client.get_markets().await;

    let perps = res["perps"]
        .as_array()
        .map(|a| a.as_slice())
        .unwrap_or(&[]);
    let spot = res["spot"]
        .as_array()
        .map(|a| a.as_slice())
        .unwrap_or(&[]);

    let mut hip3_all: Vec<Value> = Vec::new();
    if let Some(hip3_map) = res["hip3"].as_object() {
        for (dex, markets) in hip3_map {
            if let Some(arr) = markets.as_array() {
                for m in arr {
                    let mut entry = m.clone();
                    entry["dex"] = Value::String(dex.clone());
                    hip3_all.push(entry);
                }
            }
        }
    }

    println!(
        "Perps: {}  |  Spot: {}  |  HIP-3: {}\n",
        perps.len(),
        spot.len(),
        hip3_all.len()
    );

    print_group("Perps", perps);
    print_group("Spot", spot);

    if !hip3_all.is_empty() {
        println!("--- HIP-3 ---");
        for m in hip3_all.iter().take(10) {
            let dex = m["dex"].as_str().unwrap_or("?");
            let name = m["name"].as_str().unwrap_or("?");
            let display = format!("{dex}:{name}");
            println!("  {display:<16}  index={}  szDecimals={}", m["index"], m["szDecimals"]);
        }
        if hip3_all.len() > 10 {
            println!("  ... and {} more", hip3_all.len() - 10);
        }
        println!();
    }
}

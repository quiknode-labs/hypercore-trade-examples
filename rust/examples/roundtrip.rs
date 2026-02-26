use hyperliquid_api_examples::Client;
use serde_json::json;

const COIN: &str = "BTC";

async fn send_order(client: &Client, side: &str, px: &str, sz: &str) -> serde_json::Value {
    let res = client
        .exchange(&json!({
            "action": {
                "type": "order",
                "orders": [{"asset": COIN, "side": side, "price": px, "size": sz, "tif": "ioc"}],
            },
        }))
        .await;

    let hash = res["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    client
        .exchange(&json!({
            "action": res["action"],
            "nonce": res["nonce"],
            "signature": sig,
        }))
        .await
}

fn check_statuses(resp: &serde_json::Value, label: &str) -> bool {
    if let Some(statuses) = resp["response"]["data"]["statuses"].as_array() {
        for s in statuses {
            if let Some(err) = s.get("error") {
                eprintln!("{label} error: {err}");
                return false;
            }
        }
    }
    true
}

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let mid = client.get_mid(COIN).await;
    if mid == 0.0 {
        eprintln!("Could not fetch {COIN} mid price");
        std::process::exit(1);
    }

    let sz = format!("{:.5}", 11.0 / mid);
    println!("{COIN} mid: ${mid:.2}");
    println!(
        "Trade size: {sz} {COIN} (~${:.2})\n",
        sz.parse::<f64>().unwrap() * mid
    );

    let buy_px = format!("{}", (mid * 1.03) as u64);
    println!("BUY {sz} @ {buy_px} (IOC)");
    let buy_result = send_order(&client, "buy", &buy_px, &sz).await;
    let buy_resp = &buy_result["exchangeResponse"];
    if !check_statuses(buy_resp, "BUY") {
        std::process::exit(1);
    }
    println!(
        "Buy filled: {}\n",
        serde_json::to_string_pretty(buy_resp).unwrap()
    );

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let sell_px = format!("{}", (mid * 0.97) as u64);
    println!("SELL {sz} @ {sell_px} (IOC)");
    let sell_result = send_order(&client, "sell", &sell_px, &sz).await;
    let sell_resp = &sell_result["exchangeResponse"];
    if !check_statuses(sell_resp, "SELL") {
        std::process::exit(1);
    }
    println!(
        "Sell filled: {}",
        serde_json::to_string_pretty(sell_resp).unwrap()
    );
    println!("\nRound-trip complete. Position should be flat.");
}

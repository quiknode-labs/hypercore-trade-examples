use hyperliquid_api_examples::Client;
use serde_json::json;

const COIN: &str = "BTC";

async fn send_order(client: &Client, is_buy: bool, px: &str, sz: &str) -> serde_json::Value {
    let action = json!({
        "type": "order",
        "orders": [{"a": COIN, "b": is_buy, "p": px, "s": sz, "r": false, "t": {"limit": {"tif": "Ioc"}}}],
        "grouping": "na",
    });

    let res = client.rpc("hl_buildOrder", json!({"action": action})).await;

    let hash = res["result"]["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    let resolved = if res["result"]["action"].is_object() {
        res["result"]["action"].clone()
    } else {
        action
    };

    client
        .rpc(
            "hl_sendOrder",
            json!({
                "action": resolved,
                "nonce": res["result"]["nonce"],
                "signature": sig,
            }),
        )
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
    let buy_result = send_order(&client, true, &buy_px, &sz).await;
    let buy_resp = &buy_result["result"]["exchangeResponse"];
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
    let sell_result = send_order(&client, false, &sell_px, &sz).await;
    let sell_resp = &sell_result["result"]["exchangeResponse"];
    if !check_statuses(sell_resp, "SELL") {
        std::process::exit(1);
    }
    println!(
        "Sell filled: {}",
        serde_json::to_string_pretty(sell_resp).unwrap()
    );
    println!("\nRound-trip complete. Position should be flat.");
}

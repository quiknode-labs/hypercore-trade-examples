use hyperliquid_api_examples::Client;
use serde_json::json;

const COIN: &str = "BTC";

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let mid = client.get_mid(COIN).await;
    if mid == 0.0 {
        eprintln!("Could not fetch {COIN} mid price");
        std::process::exit(1);
    }

    let sz = format!("{:.5}", 11.0 / mid);
    let buy_px = format!("{}", (mid * 1.03) as u64);

    println!("{COIN} mid: ${mid:.2}");
    println!("BUY {sz} @ {buy_px} (IOC)");

    let action = json!({
        "type": "order",
        "orders": [{"a": COIN, "b": true, "p": buy_px, "s": sz, "r": false, "t": {"limit": {"tif": "Ioc"}}}],
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

    let result = client
        .rpc(
            "hl_sendOrder",
            json!({
                "action": resolved,
                "nonce": res["result"]["nonce"],
                "signature": sig,
            }),
        )
        .await;

    println!(
        "{}",
        serde_json::to_string_pretty(&result["result"]["exchangeResponse"]).unwrap()
    );
}

use hyperliquid_api_examples::Client;
use serde_json::json;

const COIN: &str = "xyz:SILVER";

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let mut mid = client.get_hip3_mid(COIN).await;
    if mid == 0.0 {
        println!("Could not fetch {COIN} mid price, using fallback");
        mid = 78.0;
    }

    let sz = format!("{:.2}", 11.0 / mid);
    let buy_px = format!("{:.2}", mid * 1.03);

    println!("{COIN} mid: ${mid:.2}");
    println!("BUY {sz} @ {buy_px} (IOC, ~${:.2} notional)\n", sz.parse::<f64>().unwrap() * mid);

    let res = client
        .exchange(&json!({
            "action": {
                "type": "order",
                "orders": [{"asset": COIN, "side": "buy", "price": buy_px, "size": sz, "tif": "ioc"}],
            },
        }))
        .await;

    let hash = res["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    let result = client
        .exchange(&json!({
            "action": res["action"],
            "nonce": res["nonce"],
            "signature": sig,
        }))
        .await;

    println!(
        "{}",
        serde_json::to_string_pretty(&result["exchangeResponse"]).unwrap()
    );
}

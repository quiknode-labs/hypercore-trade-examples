use hyperliquid_api_examples::Client;
use serde_json::json;

const COIN: &str = "BTC";
const SIZE: &str = "0.00011";

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    println!("Market BUY {SIZE} {COIN}\n");

    let res = client
        .exchange(&json!({
            "action": {
                "type": "order",
                "orders": [{"asset": COIN, "side": "buy", "size": SIZE, "tif": "market"}],
            },
        }))
        .await;

    let computed_price = res["action"]["orders"][0]["p"].as_str().unwrap_or("?");
    println!("Computed price (mid + 3% slippage): {computed_price}");
    println!("Builder fee: {}", res["builderFee"]);

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

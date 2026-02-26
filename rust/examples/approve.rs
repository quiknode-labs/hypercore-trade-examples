use hyperliquid_api_examples::Client;
use serde_json::json;

const MAX_FEE: &str = "1%";

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let res = client
        .exchange(&json!({
            "action": {"type": "approveBuilderFee", "maxFeeRate": MAX_FEE},
        }))
        .await;

    let hash = res["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    client
        .exchange(&json!({
            "action": {"type": "approveBuilderFee", "maxFeeRate": MAX_FEE},
            "nonce": res["nonce"],
            "signature": sig,
        }))
        .await;

    println!("Builder fee approved.");
}

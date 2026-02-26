use hyperliquid_api_examples::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let res = client
        .exchange(&json!({
            "action": {"type": "approveBuilderFee", "maxFeeRate": "0%"},
        }))
        .await;

    let hash = res["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    client
        .exchange(&json!({
            "action": {"type": "approveBuilderFee", "maxFeeRate": "0%"},
            "nonce": res["nonce"],
            "signature": sig,
        }))
        .await;

    println!("Builder fee revoked.");
}

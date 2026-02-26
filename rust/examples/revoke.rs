use hyperliquid_api_examples::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let res = client.rpc("hl_buildRevokeBuilderFee", json!({})).await;

    let hash = res["result"]["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    client
        .rpc(
            "hl_sendRevocation",
            json!({
                "nonce": res["result"]["nonce"],
                "signature": sig,
                "maxFeeRate": "0%",
            }),
        )
        .await;

    println!("Builder fee revoked.");
}

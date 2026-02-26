use hyperliquid_api_examples::Client;
use serde_json::json;

const MAX_FEE: &str = "1%";

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let res = client
        .rpc("hl_buildApproveBuilderFee", json!({"maxFeeRate": MAX_FEE}))
        .await;

    let hash = res["result"]["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    client
        .rpc(
            "hl_sendApproval",
            json!({
                "nonce": res["result"]["nonce"],
                "signature": sig,
                "maxFeeRate": MAX_FEE,
            }),
        )
        .await;

    println!("Builder fee approved.");
}

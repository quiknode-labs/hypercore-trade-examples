use hyperliquid_api_examples::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let res = client
        .rpc(
            "hl_getMaxBuilderFee",
            json!({"user": format!("{}", client.signer.address())}),
        )
        .await;

    println!("{}", serde_json::to_string_pretty(&res["result"]).unwrap());
}

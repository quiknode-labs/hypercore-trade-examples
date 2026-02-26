use hyperliquid_api_examples::Client;
use serde_json::json;

const COIN: &str = "HYPE";

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    println!("Closing {COIN} position for {}\n", client.address);

    let res = client
        .exchange(&json!({
            "action": {
                "type": "closePosition",
                "asset": COIN,
                "user": client.address,
            },
        }))
        .await;

    if let Some(ctx) = res.get("closePositionContext") {
        println!(
            "Position: {} {}",
            ctx["positionSize"], ctx["positionSide"]
        );
        println!(
            "Close: {} {} @ {}",
            ctx["closeSide"], ctx["closeSize"], ctx["slippedPrice"]
        );
    }

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
    println!("\nPosition closed.");
}

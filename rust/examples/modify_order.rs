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
    let rest_px = format!("{}", (mid * 0.97) as u64);

    println!("{COIN} mid: ${mid:.2}");
    println!("Placing resting BUY {sz} @ {rest_px} (GTC, 3% below mid)\n");

    // Place resting order
    let res = client
        .exchange(&json!({
            "action": {
                "type": "order",
                "orders": [{"asset": COIN, "side": "buy", "price": rest_px, "size": sz, "tif": "gtc"}],
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

    let exchange_resp = &result["exchangeResponse"];
    let statuses = exchange_resp["response"]["data"]["statuses"]
        .as_array()
        .expect("No statuses in response");

    let oid = statuses
        .iter()
        .find_map(|s| s["resting"]["oid"].as_u64())
        .expect("Could not extract OID from resting order");

    let new_px = format!("{}", (mid * 0.96) as u64);
    println!("Order resting (OID: {oid})");
    println!("Modifying price: {rest_px} -> {new_px}\n");

    // Modify order
    let modify_action = json!({
        "type": "batchModify",
        "modifies": [{
            "oid": oid,
            "order": {"asset": COIN, "side": "buy", "price": new_px, "size": sz, "tif": "gtc"},
        }],
    });

    let res = client.exchange(&json!({"action": modify_action})).await;

    let hash = res["hash"].as_str().unwrap();
    let sig = client.sign_hash(hash).await;

    let modify_result = client
        .exchange(&json!({
            "action": modify_action,
            "nonce": res["nonce"],
            "signature": sig,
        }))
        .await;

    println!(
        "{}",
        serde_json::to_string_pretty(&modify_result["exchangeResponse"]).unwrap()
    );
    println!("\nOrder modified.");
}

use hyperliquid_api_examples::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let result = client
        .post_endpoint("/openOrders", &json!({"user": client.address}))
        .await;

    let count = result["count"].as_u64().unwrap_or(0);
    println!("Open orders: {count}");

    if let Some(orders) = result["orders"].as_array() {
        for order in orders {
            let name = order["name"].as_str().unwrap_or("?");
            let side = if order["side"].as_str() == Some("B") { "BUY" } else { "SELL" };
            let spot = if order["isSpot"].as_bool() == Some(true) { " [SPOT]" } else { "" };
            let sz = order["sz"].as_str().unwrap_or("?");
            let px = order["limitPx"].as_str().unwrap_or("?");
            let oid = &order["oid"];
            println!("  {name}{spot} {side} {sz} @ {px} (OID: {oid})");
        }
    }

    if count > 0 {
        println!("\nTo cancel ALL orders, pass cancelActions.all as action to POST /exchange:");
        println!(
            "{}",
            serde_json::to_string_pretty(&result["cancelActions"]["all"]).unwrap()
        );
    }
}

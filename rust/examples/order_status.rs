use hyperliquid_api_examples::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let oid: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if oid == 0 {
        eprintln!("Usage: cargo run --example order_status -- <oid>");
        eprintln!("Get OIDs from: cargo run --example open_orders");
        std::process::exit(1);
    }

    let res = client
        .rpc(
            "hl_orderStatus",
            json!({"user": format!("{}", client.signer.address()), "oid": oid}),
        )
        .await;

    let result = &res["result"];
    let status = result["status"].as_str().unwrap_or("?");
    let explanation = result["explanation"].as_str().unwrap_or("");

    if status == "unknownOid" {
        println!("Order {oid}: not found");
        println!("  {explanation}");
        return;
    }

    let name = result["name"].as_str().unwrap_or("?");
    let spot = if result["isSpot"].as_bool() == Some(true) { " [SPOT]" } else { "" };

    println!("Order {oid} on {name}{spot}: {status}");
    println!("  {explanation}");
}

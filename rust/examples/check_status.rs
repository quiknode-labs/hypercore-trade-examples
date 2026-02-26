use hyperliquid_api_examples::Client;

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let res = client.get_approval(&client.address).await;

    println!("{}", serde_json::to_string_pretty(&res).unwrap());
}

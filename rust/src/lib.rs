use alloy::primitives::B256;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use serde_json::{json, Value};
use std::env;

pub struct Client {
    pub endpoint: String,
    pub signer: PrivateKeySigner,
    http: reqwest::Client,
    req_id: std::sync::atomic::AtomicU64,
}

impl Client {
    pub fn from_env() -> Self {
        let pk = env::var("PRIVATE_KEY").expect("Set PRIVATE_KEY environment variable");
        let pk_bytes = hex::decode(pk.trim_start_matches("0x")).expect("Invalid PRIVATE_KEY hex");
        let signer = PrivateKeySigner::from_slice(&pk_bytes).expect("Invalid private key");

        let endpoint = "https://send.hyperliquidapi.com".to_string();

        println!("Wallet: {}", signer.address());

        Self {
            endpoint,
            signer,
            http: reqwest::Client::new(),
            req_id: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub async fn rpc(&self, method: &str, params: Value) -> Value {
        let id = self
            .req_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id,
        });

        let resp = self
            .http
            .post(&self.endpoint)
            .json(&body)
            .send()
            .await
            .expect("HTTP request failed");

        let data: Value = resp.json().await.expect("Invalid JSON response");

        if let Some(err) = data.get("error") {
            eprintln!("\nRPC error ({method}):");
            eprintln!("  code:     {}", err.get("code").unwrap_or(&Value::Null));
            eprintln!("  message:  {}", err.get("message").unwrap_or(&Value::Null));
            if let Some(guidance) = err.pointer("/data/guidance") {
                eprintln!("  guidance: {guidance}");
            }
            std::process::exit(1);
        }

        data
    }

    pub async fn sign_hash(&self, hash_hex: &str) -> Value {
        let hash_bytes = hex::decode(hash_hex.trim_start_matches("0x")).expect("Invalid hash hex");
        let hash = B256::from_slice(&hash_bytes);

        let sig = self
            .signer
            .sign_hash(&hash)
            .await
            .expect("Signing failed");

        let v = sig.v() as u8;
        let v_legacy = if v < 27 { v + 27 } else { v };
        json!({
            "r": format!("0x{:064x}", sig.r()),
            "s": format!("0x{:064x}", sig.s()),
            "v": v_legacy,
        })
    }

    pub async fn get_mid(&self, coin: &str) -> f64 {
        let resp = self
            .http
            .post("https://api.hyperliquid.xyz/info")
            .json(&json!({"type": "allMids"}))
            .send()
            .await
            .expect("Failed to fetch mids");

        let data: Value = resp.json().await.expect("Invalid mids JSON");
        data.get(coin)
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0)
    }
}

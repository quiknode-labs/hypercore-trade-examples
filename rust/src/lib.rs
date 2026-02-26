use alloy::primitives::B256;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use serde_json::{json, Value};
use std::env;

const API_URL: &str = "https://send.hyperliquidapi.com";
const HL_INFO_URL: &str = "https://api.hyperliquid.xyz/info";

pub struct Client {
    pub signer: PrivateKeySigner,
    pub address: String,
    http: reqwest::Client,
}

impl Client {
    pub fn from_env() -> Self {
        let pk = env::var("PRIVATE_KEY").expect("Set PRIVATE_KEY environment variable");
        let pk_bytes = hex::decode(pk.trim_start_matches("0x")).expect("Invalid PRIVATE_KEY hex");
        let signer = PrivateKeySigner::from_slice(&pk_bytes).expect("Invalid private key");

        let address = format!("{}", signer.address());
        println!("Wallet: {address}");

        Self {
            signer,
            address,
            http: reqwest::Client::new(),
        }
    }

    pub async fn exchange(&self, body: &Value) -> Value {
        let resp = self
            .http
            .post(format!("{API_URL}/exchange"))
            .json(body)
            .send()
            .await
            .expect("HTTP request failed");

        let status = resp.status();
        let data: Value = resp.json().await.expect("Invalid JSON response");

        if data.get("error").is_some() {
            eprintln!("\nError ({status}):");
            eprintln!("  error:    {}", data.get("error").unwrap_or(&Value::Null));
            eprintln!("  message:  {}", data.get("message").unwrap_or(&Value::Null));
            if let Some(guidance) = data.get("guidance") {
                eprintln!("  guidance: {guidance}");
            }
            std::process::exit(1);
        }

        data
    }

    pub async fn get_approval(&self, user: &str) -> Value {
        let resp = self
            .http
            .get(format!("{API_URL}/approval?user={user}"))
            .send()
            .await
            .expect("HTTP request failed");

        resp.json().await.expect("Invalid JSON response")
    }

    pub async fn get_markets(&self) -> Value {
        let resp = self
            .http
            .get(format!("{API_URL}/markets"))
            .send()
            .await
            .expect("HTTP request failed");

        resp.json().await.expect("Invalid JSON response")
    }

    pub async fn post_endpoint(&self, path: &str, body: &Value) -> Value {
        let resp = self
            .http
            .post(format!("{API_URL}{path}"))
            .json(body)
            .send()
            .await
            .expect("HTTP request failed");

        resp.json().await.expect("Invalid JSON response")
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
            .post(HL_INFO_URL)
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

    pub async fn get_hip3_mid(&self, coin: &str) -> f64 {
        let dex = coin.split(':').next().unwrap_or("");
        let resp = self
            .http
            .post(HL_INFO_URL)
            .json(&json!({"type": "allMids", "dex": dex}))
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

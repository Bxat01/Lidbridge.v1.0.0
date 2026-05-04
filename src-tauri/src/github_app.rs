// LidBridge — Open-Source Desktop Tool for Cleaning and Publishing Projects to GitHub
// Copyright (C) 2026 Lidprex Labs <https://lidprex.onrender.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;

pub const APP_ID: &str = "3522405";

// NOTE: Place your GitHub App private key at src/keys/private-key.pem
// See README.md for setup instructions.
pub const PRIVATE_KEY: &str = include_str!("keys/private-key.pem");

#[derive(Debug, Serialize)]
struct JWTClaims {
    iat: u64,
    exp: u64,
    iss: String,
}

pub async fn get_installation_token(installation_id: &str) -> Result<String, String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let claims = JWTClaims {
        iat: now,
        exp: now + (10 * 60),
        iss: APP_ID.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_rsa_pem(PRIVATE_KEY.as_bytes()).unwrap()
    ).unwrap();

    let client = Client::new();
    let response = client
        .post(&format!("https://api.github.com/app/installations/{}/access_tokens", installation_id))
        .bearer_auth(token)
        .header("User-Agent", "LidBridge")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("Failed to get installation token: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API error: {}", error_text));
    }

    let json: serde_json::Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
    json["token"].as_str().map(|s| s.to_string()).ok_or("No token in response".to_string())
}

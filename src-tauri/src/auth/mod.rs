// LidBridge — Open-Source Desktop Tool for Cleaning and Publishing Projects to GitHub
// Copyright (C) 2026 Lidprex Labs <https://lidprex.onrender.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use rand::Rng;
use hex;

const FALLBACK_CLIENT_ID: &str = "Ov23liKi1YFqzKBc2Usi";
const REDIRECT_URI: &str = "http://localhost:2026/callback";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
}

pub struct AuthManager {
    csrf_token: Mutex<Option<String>>,
    client_id: String,
    client_secret: String,
}

impl AuthManager {
    pub fn new() -> Self {
        let client_id = option_env!("GITHUB_CLIENT_ID")
            .unwrap_or(FALLBACK_CLIENT_ID)
            .to_string();

        let client_secret = option_env!("GITHUB_CLIENT_SECRET")
            .unwrap_or("")
            .to_string();

        Self {
            csrf_token: Mutex::new(None),
            client_id,
            client_secret,
        }
    }

    pub fn get_authorization_url(&self) -> (String, String) {
        let mut rng = rand::thread_rng();
        let csrf = hex::encode(rng.gen::<[u8; 32]>());

        *self.csrf_token.lock().unwrap() = Some(csrf.clone());

        let scope = "repo%20write:org%20read:user";

        let auth_url = format!(
            "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}",
            self.client_id, REDIRECT_URI, scope, csrf
        );

        (auth_url, csrf)
    }

    pub async fn exchange_code_for_token(&self, code: &str) -> Result<String, String> {
        let client = reqwest::Client::new();

        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
        ];

        let response = client
            .post("https://github.com/login/oauth/access_token")
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        json["access_token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "No access token in response".to_string())
    }

    pub async fn get_github_user(&self, token: &str) -> Result<(String, String, String, String), String> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "LidBridge")
            .send()
            .await
            .map_err(|e| format!("Failed to get user: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let github_id = json["id"].as_i64().unwrap_or(0).to_string();
        let email = json["email"].as_str().unwrap_or("").to_string();
        let name = json["login"].as_str().unwrap_or("").to_string();
        let avatar_url = json["avatar_url"].as_str().unwrap_or("").to_string();

        Ok((github_id, email, name, avatar_url))
    }
}

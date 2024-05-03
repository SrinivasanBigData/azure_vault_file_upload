use std::fs;
use reqwest::Client;
use serde::{ Serialize, Deserialize };
use log::{ info, error };

#[derive(Serialize, Deserialize, Debug)]
struct SecretVaule {
    value: String,
}

pub async fn upload_file_to_vault(
    token: &str,
    vault_url: &str,
    api_version: &str,
    secret_name: String,
    file_path: String
) -> Result<(), Box<dyn std::error::Error>> {
    let private_key = fs::read_to_string(file_path)?;
    let secret_value = SecretVaule {
        value: private_key,
    };
    let secret_payload = serde_json::to_string(&secret_value).unwrap();
    let client = Client::new();
    let upload_url = format!("{}secrets/{}?api-version={}", vault_url, secret_name, api_version);
    info!("Uploaded Secret URL: {}", &upload_url);
    let response = client
        .put(upload_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(secret_payload)
        .send().await?;
    if response.status().is_success() {
        info!("Private key uploaded successfully!");
    } else {
        error!("Failed to upload private key: {:?}", response.text().await?);
    }
    Ok(())
}

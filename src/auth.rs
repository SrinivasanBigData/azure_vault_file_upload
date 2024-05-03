use reqwest::Client;
use log::info;

use crate::config::Config;

/**
 * Get the Access Token from Azure Vault
 */
pub async fn get_access_token(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    info!("Getting Access Token.");
    let client = Client::new();
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret),
        ("scope", "https://vault.azure.net/.default"),
    ];
    let auth_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        &config.tenant_id
    );
    info!("Login Url:{}", &auth_url);
    let response = client
        .post(auth_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send().await?;
    let json_value = response.json::<serde_json::Value>().await?;
    let token = json_value["access_token"]
        .as_str()
        .ok_or("Invalid credentials: Access token not found")?;
    Ok(token.to_string())
}

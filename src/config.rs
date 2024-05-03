use std::env;
use dotenv::dotenv;
use log::{ error, info };

#[derive(Debug)]
pub(crate) struct Config {
    pub vault_url: String,
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub api_version: String,
}

impl Config {
    /**
     *Read the configuration from environment variables
     */
    pub fn from_env() -> Result<Self, String> {
        info!("Reading the values from environment");
        dotenv().ok();
        let vault_url = env::var("VAULT_URL").map_err(|_| "VAULT_URL not found in .env")?;
        print!("{}", vault_url);
        let tenant_id = env::var("TENANT_ID").map_err(|_| "TENANT_ID not found in .env")?;
        let client_id = env::var("CLIENT_ID").map_err(|_| "CLIENT_ID not found in .env")?;
        let client_secret = env
            ::var("CLIENT_SECRET")
            .map_err(|_| "CLIENT_SECRET not found in .env")?;
        let api_version: String = env
            ::var("API_VERSION")
            .map(|version| if version.is_empty() { "7.4".to_string() } else { version })
            .unwrap_or("7.4".to_string());

        if vault_url.is_empty() {
            return Err("VAULT_URL cannot be empty".to_string());
        }
        if tenant_id.is_empty() {
            return Err("TENANT_ID cannot be empty".to_string());
        }
        if client_id.is_empty() {
            return Err("CLIENT_ID cannot be empty".to_string());
        }
        if client_secret.is_empty() {
            return Err("CLIENT_SECRET cannot be empty".to_string());
        }
        Ok(Config {
            vault_url,
            tenant_id,
            client_id,
            client_secret,
            api_version,
        })
    }

    /**
     * Handle the config Result
     */
    pub fn handle_config_result(result: Result<Config, String>) -> Self {
        match result {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error: {}", err);
                error!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}

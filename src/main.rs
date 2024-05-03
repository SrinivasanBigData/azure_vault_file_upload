mod auth;
mod vault;
mod config;

use std::{ error::Error, path::Path };
use auth::get_access_token;
use vault::upload_file_to_vault;
use clap::Parser;
use config::Config;
use log::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    secret_name: String,

    #[arg(short, long)]
    file_path: String,
}

fn parse_and_validate_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();
    if !Path::new(&args.file_path).exists() {
        return Err(format!("Error: File '{}' does not exist!", &args.file_path).into());
    }
    Ok(args)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting application");
    let Args { secret_name, file_path } = parse_and_validate_args()?;
    let config: Config = Config::handle_config_result(Config::from_env());
    let token = get_access_token(&config).await?;
    info!("add/update the secret key value: {} and file {}", &secret_name, &file_path);
    upload_file_to_vault(
        &token,
        &config.vault_url,
        &config.api_version,
        secret_name,
        file_path
    ).await?;
    Ok(())
}

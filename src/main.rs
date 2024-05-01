use anyhow::Result;
use config::CONFIG;
use reqwest::header;
use serde_json::Value;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

pub mod config;

#[tokio::main]
async fn main() -> Result<()> {
    Registry::default()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();
    tracing::info!(env = CONFIG.environment.to_string(), "starting up");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        header::USER_AGENT,
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"))
            .parse()
            .unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let res = client
        .get("https://ip.fyko.net/json")
        .header(header::ACCEPT, "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    tracing::info!("IP information: {res:#?}");

    Ok(())
}

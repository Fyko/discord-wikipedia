use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Environment {
    /// Development environment
    #[serde(rename = "development")]
    Development,
    /// Production environment
    #[serde(rename = "production")]
    Production,
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Development => "development".to_string(),
            Environment::Production => "production".to_string(),
        }
    }
}

/// Application Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The environment the application is running in
    pub environment: Environment,
}

impl Config {
    /// Create a new `Config`
    pub fn new() -> Result<Self> {
        let config = envy::from_env::<Self>()?;

        Ok(config)
    }
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
    &CONFIG
}

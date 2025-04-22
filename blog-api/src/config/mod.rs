use config::{Config, ConfigError, File};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());
        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config".into());

        let settings = Config::builder()
            .add_source(File::with_name(&format!("{}/default", config_path)))
            .add_source(File::with_name(&format!("{}/{}", config_path, environment)).required(false))
            // Add in environment variables with a prefix of APP and '__' as separator
            // For example: APP_SERVER__PORT=8080
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?
            .try_deserialize::<AppConfig>()?;

        Ok(settings)
    }

    pub fn default_development() -> Self {
        AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost:5432/blog_db".to_string(),
            },
        }
    }
}
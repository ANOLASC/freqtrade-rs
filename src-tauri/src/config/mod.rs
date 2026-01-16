use crate::error::{AppError, Result};
use crate::types::Timeframe;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    pub max_open_trades: usize,
    pub stake_currency: String,
    pub stake_amount: f64,
    pub dry_run: bool,
    pub dry_run_wallet: f64,
    pub process_only_new_candles: bool,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            max_open_trades: 3,
            stake_currency: "USDT".to_string(),
            stake_amount: 100.0,
            dry_run: true,
            dry_run_wallet: 10000.0,
            process_only_new_candles: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeConfig {
    pub name: String,
    pub key: String,
    pub secret: String,
    pub enable_rate_limit: bool,
}

impl Default for ExchangeConfig {
    fn default() -> Self {
        Self {
            name: "binance".to_string(),
            key: String::new(),
            secret: String::new(),
            enable_rate_limit: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub name: String,
    pub timeframe: Timeframe,
    pub params: serde_json::Value,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            name: "SimpleStrategy".to_string(),
            timeframe: Timeframe::OneHour,
            params: serde_json::json!({}),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "user_data/trades.db".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServerConfig {
    pub enabled: bool,
    pub listen_ip: String,
    pub listen_port: u16,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            listen_ip: "127.0.0.1".to_string(),
            listen_port: 8080,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "INFO".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub bot: BotConfig,
    #[serde(default)]
    pub exchange: ExchangeConfig,
    #[serde(default)]
    pub strategy: StrategyConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub api_server: ApiServerConfig,
    #[serde(default)]
    pub log: LogConfig,
}

pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .await
            .map_err(|e| AppError::Config(format!("Failed to read config file: {}", e)))?;

        let config: AppConfig =
            toml::from_str(&content).map_err(|e| AppError::Config(format!("Failed to parse config: {}", e)))?;

        Ok(Self { config })
    }

    pub async fn load_from_env(&mut self) -> Result<()> {
        if let Ok(key) = std::env::var("EXCHANGE_API_KEY") {
            self.config.exchange.key = key;
        }
        if let Ok(secret) = std::env::var("EXCHANGE_API_SECRET") {
            self.config.exchange.secret = secret;
        }
        if let Ok(db_path) = std::env::var("DATABASE_PATH") {
            self.config.database.path = db_path;
        }
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            self.config.log.level = log_level;
        }
        Ok(())
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(path.as_ref(), content)
            .await
            .map_err(|e| AppError::Config(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.bot.max_open_trades, 3);
        assert_eq!(config.bot.dry_run, true);
        assert_eq!(config.exchange.name, "binance");
    }

    #[test]
    fn test_parse_config_from_string() {
        let config_str = r#"
        [bot]
        max_open_trades = 5
        dry_run = false

        [exchange]
        name = "binance"
        key = "test_key"
        "#;

        let config: AppConfig = toml::from_str(config_str).unwrap();
        assert_eq!(config.bot.max_open_trades, 5);
        assert_eq!(config.bot.dry_run, false);
        assert_eq!(config.exchange.key, "test_key");
    }
}

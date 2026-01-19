#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, ConfigManager};

    #[tokio::test]
    async fn test_config_manager_creation() {
        let config = AppConfig::default();
        let manager = ConfigManager::new(config);

        assert_eq!(manager.config().bot.stake_currency, "USDT");
        assert_eq!(manager.config().exchange.name, "binance");
    }

}

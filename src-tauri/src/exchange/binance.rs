use crate::error::AppError;
use crate::error::Result;
use crate::exchange::Exchange;
use crate::types::*;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;

pub struct BinanceExchange {
    api_key: String,
    api_secret: String,
    client: reqwest::Client,
}

impl BinanceExchange {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            client: reqwest::Client::new(),
        }
    }

    fn get_base_url(&self) -> &str {
        "https://api.binance.com"
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    async fn fetch_ticker(&self, symbol: &str) -> Result<Ticker> {
        let url = format!("{}/api/v3/ticker/24hr?symbol={}", self.get_base_url(), symbol);
        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;
        
        Ok(Ticker {
            symbol: data["symbol"].as_str().unwrap_or("").to_string(),
            price: data["lastPrice"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO),
            volume_24h: data["volume"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO),
            change_24h: data["priceChangePercent"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO),
        })
    }

    async fn fetch_ohlcv(&self, symbol: &str, timeframe: &str, limit: usize) -> Result<Vec<OHLCV>> {
        let url = format!(
            "{}/api/v3/klines?symbol={}&interval={}&limit={}",
            self.get_base_url(),
            symbol,
            timeframe,
            limit
        );
        
        let response = self.client.get(&url).send().await?;
        let data: Vec<serde_json::Value> = response.json().await?;
        
        let mut klines = Vec::new();
        for item in data {
            if let (Some(open_time), Some(open), Some(high), Some(low), Some(close), Some(volume)) = (
                item[0].as_i64(),
                item[1].as_str(),
                item[2].as_str(),
                item[3].as_str(),
                item[4].as_str(),
                item[5].as_str(),
            ) {
                let timestamp = chrono::DateTime::from_timestamp(open_time / 1000, 0)
                    .unwrap_or_else(|| Utc::now());
                
                klines.push(OHLCV {
                    timestamp,
                    open: open.parse().unwrap_or(Decimal::ZERO),
                    high: high.parse().unwrap_or(Decimal::ZERO),
                    low: low.parse().unwrap_or(Decimal::ZERO),
                    close: close.parse().unwrap_or(Decimal::ZERO),
                    volume: volume.parse().unwrap_or(Decimal::ZERO),
                });
            }
        }
        
        Ok(klines)
    }

    async fn fetch_balance(&self) -> Result<Balance> {
        Err(AppError::NotImplemented("fetch_balance not implemented yet".to_string()))
    }

    async fn fetch_positions(&self) -> Result<Vec<Position>> {
        Err(AppError::NotImplemented("fetch_positions not implemented yet".to_string()))
    }

    async fn create_order(&self, _order: OrderRequest) -> Result<Order> {
        Err(AppError::NotImplemented("create_order not implemented yet".to_string()))
    }

    async fn cancel_order(&self, _order_id: &str) -> Result<()> {
        Err(AppError::NotImplemented("cancel_order not implemented yet".to_string()))
    }

    async fn fetch_order(&self, _order_id: &str) -> Result<Order> {
        Err(AppError::NotImplemented("fetch_order not implemented yet".to_string()))
    }

    async fn fetch_orders(&self, _symbol: &str) -> Result<Vec<Order>> {
        Err(AppError::NotImplemented("fetch_orders not implemented yet".to_string()))
    }

    fn get_name(&self) -> &str {
        "binance"
    }
}

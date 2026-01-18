use crate::error::AppError;
use crate::error::Result;
use crate::exchange::Exchange;
use crate::types::*;
use async_trait::async_trait;
use chrono::Utc;
use hmac::{Hmac, Mac};
use rust_decimal::Decimal;
use serde::Deserialize;
use sha2::Sha256;

#[derive(Deserialize)]
struct BinanceOrderResponse {
    symbol: String,
    #[serde(rename = "orderId")]
    order_id: i64,
    #[serde(rename = "transactTime")]
    transact_time: i64,
    #[serde(rename = "origQty")]
    orig_qty: String,
    #[serde(rename = "executedQty")]
    executed_qty: String,
    status: String,
}

pub struct BinanceExchange {
    _api_key: String,
    _api_secret: String,
    base_url: String,
    client: reqwest::Client,
}

impl BinanceExchange {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            _api_key: api_key,
            _api_secret: api_secret,
            base_url: "https://api.binance.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    fn sign_query(&self, query: String) -> String {
        let timestamp = Utc::now().timestamp_millis();
        let query_with_timestamp = if query.is_empty() {
            format!("timestamp={}", timestamp)
        } else {
            format!("{}&timestamp={}", query, timestamp)
        };

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self._api_secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(query_with_timestamp.as_bytes());
        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        format!("{}&signature={}", query_with_timestamp, signature)
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
            price: data["lastPrice"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(Decimal::ZERO),
            volume_24h: data["volume"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO),
            change_24h: data["priceChangePercent"]
                .as_str()
                .unwrap_or("0")
                .parse()
                .unwrap_or(Decimal::ZERO),
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
                let timestamp = chrono::DateTime::from_timestamp(open_time / 1000, 0).unwrap_or_else(Utc::now);

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
        Err(AppError::NotImplemented(
            "fetch_balance not implemented yet".to_string(),
        ))
    }

    async fn fetch_positions(&self) -> Result<Vec<Position>> {
        Err(AppError::NotImplemented(
            "fetch_positions not implemented yet".to_string(),
        ))
    }

    async fn create_order(&self, order: OrderRequest) -> Result<Order> {
        let side = match order.side {
            TradeSide::Buy => "BUY",
            TradeSide::Sell => "SELL",
        };

        let order_type = match order.order_type {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::StopLimit => "STOP_LOSS_LIMIT",
            OrderType::StopMarket => "STOP_LOSS",
        };

        let mut query = format!(
            "symbol={}&side={}&type={}&quantity={}",
            order.symbol, side, order_type, order.amount
        );

        if let Some(price) = order.price {
            if order.order_type != OrderType::Market {
                query.push_str(&format!("&price={}", price));
                query.push_str("&timeInForce=GTC");
            }
        } else if order.order_type == OrderType::Limit {
            return Err(AppError::Exchange("Limit order requires price".to_string()));
        }

        let signed_query = self.sign_query(query);
        let url = format!("{}/api/v3/order?{}", self.get_base_url(), signed_query);

        let response = self
            .client
            .post(&url)
            .header("X-MBX-APIKEY", &self._api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Exchange(format!("Binance error: {}", error_text)));
        }

        let data: BinanceOrderResponse = response.json().await?;

        let status = match data.status.as_str() {
            "NEW" => OrderStatus::New,
            "PARTIALLY_FILLED" => OrderStatus::PartiallyFilled,
            "FILLED" => OrderStatus::Filled,
            "CANCELED" => OrderStatus::Canceled,
            "REJECTED" => OrderStatus::Rejected,
            "EXPIRED" => OrderStatus::Expired,
            _ => OrderStatus::New,
        };

        let amount = data.orig_qty.parse().unwrap_or(Decimal::ZERO);
        let filled = data.executed_qty.parse().unwrap_or(Decimal::ZERO);
        let remaining = amount - filled;

        let created_at = chrono::DateTime::from_timestamp(data.transact_time / 1000, 0).unwrap_or_else(Utc::now);

        Ok(Order {
            id: data.order_id.to_string(),
            symbol: data.symbol,
            side: order.side,
            order_type: order.order_type,
            status,
            price: order.price,
            amount,
            filled,
            remaining,
            fee: None,
            created_at,
            updated_at: created_at,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_create_order_success() {
        let mut server = Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("POST", "/api/v3/order")
            .match_header("X-MBX-APIKEY", "test_key")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "symbol": "BTCUSDT",
                "orderId": 28,
                "orderListId": -1,
                "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
                "transactTime": 1507725176595,
                "price": "0.00000000",
                "origQty": "1.00000000",
                "executedQty": "1.00000000",
                "cummulativeQuoteQty": "10.00000000",
                "status": "FILLED",
                "timeInForce": "GTC",
                "type": "MARKET",
                "side": "BUY"
            }"#,
            )
            .create_async()
            .await;

        let exchange = BinanceExchange::new("test_key".to_string(), "test_secret".to_string()).with_base_url(url);

        let order_req = OrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: TradeSide::Buy,
            order_type: OrderType::Market,
            amount: Decimal::from_str("1.0").unwrap(),
            price: None,
        };

        let result = exchange.create_order(order_req).await;

        mock.assert_async().await;
        assert!(result.is_ok());
        let order = result.unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.amount, Decimal::from_str("1.0").unwrap());
    }
}

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
    #[serde(rename = "cummulativeQuoteQty")]
    cummulative_quote_qty: String,
    status: String,
    #[serde(rename = "side")]
    side: String,
    #[serde(rename = "type")]
    order_type: String,
    #[serde(rename = "price")]
    price: String,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    update_time: i64,
    #[serde(rename = "avgPrice")]
    #[serde(default)]
    avg_price: String,
}

#[derive(Deserialize)]
struct BinanceTradeResponse {
    #[serde(rename = "orderId")]
    order_id: i64,
    #[serde(rename = "tradeId")]
    trade_id: i64,
    #[serde(rename = "symbol")]
    symbol: String,
    #[serde(rename = "price")]
    price: String,
    #[serde(rename = "qty")]
    qty: String,
    #[serde(rename = "commission")]
    commission: String,
    #[serde(rename = "commissionAsset")]
    commission_asset: String,
    #[serde(rename = "time")]
    time: i64,
    #[serde(rename = "isBuyer")]
    is_buyer: bool,
}

#[derive(Deserialize)]
struct BinanceAccountInfo {
    #[serde(rename = "makerCommission")]
    maker_commission: i64,
    #[serde(rename = "takerCommission")]
    taker_commission: i64,
    balances: Vec<BalanceInfo>,
}

#[derive(Deserialize)]
struct BalanceInfo {
    asset: String,
    free: String,
    locked: String,
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
        let query = String::new();
        let signed_query = self.sign_query(query);
        let url = format!("{}/api/v3/account?{}", self.get_base_url(), signed_query);

        let response = self
            .client
            .get(&url)
            .header("X-MBX-APIKEY", &self._api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Exchange(format!("Binance error: {}", error_text)));
        }

        let data: BinanceAccountInfo = response.json().await?;

        // Find USDT balance
        for balance in &data.balances {
            if balance.asset == "USDT" {
                let free = balance.free.parse().unwrap_or(Decimal::ZERO);
                let locked = balance.locked.parse().unwrap_or(Decimal::ZERO);
                let total = free + locked;
                return Ok(Balance {
                    currency: "USDT".to_string(),
                    total,
                    free,
                    used: locked,
                });
            }
        }

        Ok(Balance {
            currency: "USDT".to_string(),
            total: Decimal::ZERO,
            free: Decimal::ZERO,
            used: Decimal::ZERO,
        })
    }

    async fn fetch_positions(&self) -> Result<Vec<Position>> {
        let query = String::new();
        let signed_query = self.sign_query(query);
        let url = format!("{}/api/v3/account?{}", self.get_base_url(), signed_query);

        let response = self
            .client
            .get(&url)
            .header("X-MBX-APIKEY", &self._api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Exchange(format!("Binance error: {}", error_text)));
        }

        let data: serde_json::Value = response.json().await?;
        let positions_json = &data["positions"];

        let mut positions = Vec::new();
        if let Some(positions_arr) = positions_json.as_array() {
            for pos in positions_arr {
                let symbol = pos["symbol"].as_str().unwrap_or("").to_string();
                let notional = pos["notional"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO);
                let entry_price = pos["entryPrice"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO);
                let mark_price = pos["markPrice"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO);

                // Only include non-zero positions
                if notional != Decimal::ZERO {
                    let side = if notional > Decimal::ZERO { TradeSide::Buy } else { TradeSide::Sell };
                    let size = notional.abs();
                    let unrealized_pnl = pos["unrealizedProfit"].as_str().unwrap_or("0").parse().unwrap_or(Decimal::ZERO);

                    positions.push(Position {
                        symbol,
                        side,
                        size,
                        entry_price,
                        mark_price,
                        unrealized_pnl,
                        percentage: Decimal::ZERO,
                    });
                }
            }
        }

        Ok(positions)
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

    async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let query = format!("orderId={}", order_id);
        let signed_query = self.sign_query(query);
        let url = format!("{}/api/v3/order?{}", self.get_base_url(), signed_query);

        let response = self
            .client
            .delete(&url)
            .header("X-MBX-APIKEY", &self._api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Exchange(format!("Failed to cancel order: {}", error_text)));
        }

        Ok(())
    }

    async fn fetch_order(&self, order_id: &str) -> Result<Order> {
        let query = format!("orderId={}", order_id);
        let signed_query = self.sign_query(query);
        let url = format!("{}/api/v3/order?{}", self.get_base_url(), signed_query);

        let response = self
            .client
            .get(&url)
            .header("X-MBX-APIKEY", &self._api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Exchange(format!("Failed to fetch order: {}", error_text)));
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

        let side = match data.side.as_str() {
            "BUY" => TradeSide::Buy,
            "SELL" => TradeSide::Sell,
            _ => TradeSide::Buy,
        };

        let order_type = match data.order_type.as_str() {
            "MARKET" => OrderType::Market,
            "LIMIT" => OrderType::Limit,
            "STOP_LOSS_LIMIT" => OrderType::StopLimit,
            "STOP_LOSS" => OrderType::StopMarket,
            _ => OrderType::Market,
        };

        let amount = data.orig_qty.parse().unwrap_or(Decimal::ZERO);
        let filled = data.executed_qty.parse().unwrap_or(Decimal::ZERO);
        let remaining = amount - filled;
        let price = data.price.parse().unwrap_or(Decimal::ZERO);

        let created_at = chrono::DateTime::from_timestamp(data.transact_time / 1000, 0).unwrap_or_else(Utc::now);
        let updated_at = chrono::DateTime::from_timestamp(data.update_time / 1000, 0).unwrap_or_else(Utc::now);

        Ok(Order {
            id: data.order_id.to_string(),
            symbol: data.symbol,
            side,
            order_type,
            status,
            price: Some(price),
            amount,
            filled,
            remaining,
            fee: None,
            created_at,
            updated_at,
        })
    }

    async fn fetch_orders(&self, symbol: &str) -> Result<Vec<Order>> {
        let query = format!("symbol={}", symbol);
        let signed_query = self.sign_query(query);
        let url = format!("{}/api/v3/allOrders?{}", self.get_base_url(), signed_query);

        let response = self
            .client
            .get(&url)
            .header("X-MBX-APIKEY", &self._api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Exchange(format!("Failed to fetch orders: {}", error_text)));
        }

        let orders_data: Vec<BinanceOrderResponse> = response.json().await?;

        let mut orders = Vec::new();
        for data in orders_data {
            let status = match data.status.as_str() {
                "NEW" => OrderStatus::New,
                "PARTIALLY_FILLED" => OrderStatus::PartiallyFilled,
                "FILLED" => OrderStatus::Filled,
                "CANCELED" => OrderStatus::Canceled,
                "REJECTED" => OrderStatus::Rejected,
                "EXPIRED" => OrderStatus::Expired,
                _ => OrderStatus::New,
            };

            let side = match data.side.as_str() {
                "BUY" => TradeSide::Buy,
                "SELL" => TradeSide::Sell,
                _ => TradeSide::Buy,
            };

            let order_type = match data.order_type.as_str() {
                "MARKET" => OrderType::Market,
                "LIMIT" => OrderType::Limit,
                "STOP_LOSS_LIMIT" => OrderType::StopLimit,
                "STOP_LOSS" => OrderType::StopMarket,
                _ => OrderType::Market,
            };

            let amount = data.orig_qty.parse().unwrap_or(Decimal::ZERO);
            let filled = data.executed_qty.parse().unwrap_or(Decimal::ZERO);
            let remaining = amount - filled;
            let price = data.price.parse().unwrap_or(Decimal::ZERO);

            let created_at = chrono::DateTime::from_timestamp(data.transact_time / 1000, 0).unwrap_or_else(Utc::now);
            let updated_at = chrono::DateTime::from_timestamp(data.update_time / 1000, 0).unwrap_or_else(Utc::now);

            orders.push(Order {
                id: data.order_id.to_string(),
                symbol: data.symbol,
                side,
                order_type,
                status,
                price: Some(price),
                amount,
                filled,
                remaining,
                fee: None,
                created_at,
                updated_at,
            });
        }

        Ok(orders)
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

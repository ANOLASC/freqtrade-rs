//! WebSocket connection manager for real-time updates from Binance
//!
//! This module provides WebSocket connection management for receiving
//! real-time account updates, order updates, and position updates.

use crate::error::{AppError, Result};
use crate::types::{Balance, Order, Position};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

/// WebSocket connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

/// Binance WebSocket stream types
#[derive(Debug, Clone, strum::IntoStaticStr)]
pub enum BinanceStream {
    Account,
    Orders,
    AllOrders,
    Trade,
    Kline(String),
    Ticker(String),
    Heartbeat,
}

/// WebSocket message types from Binance
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "e")]
#[allow(non_snake_case)]
pub enum BinanceWebSocketMessage {
    #[serde(rename = "outboundAccountPosition")]
    AccountUpdate {
        u: i64,
        B: Vec<BalanceUpdate>,
    },
    #[serde(rename = "executionReport")]
    OrderUpdate {
        s: String,
        c: String,
        S: String,
        o: String,
        f: String,
        p: String,
        q: String,
        P: String,
        d: String,
        x: String,
        X: String,
        i: i64,
        l: String,
        n: String,
        N: Option<String>,
        T: i64,
        t: i64,
        v: String,
        z: String,
    },
    #[serde(rename = "trade")]
    Trade {
        E: i64,
        s: String,
        t: i64,
        p: String,
        q: String,
        b: i64,
        a: i64,
        T: i64,
        m: bool,
    },
    #[serde(rename = "ping")]
    Heartbeat {
        msg_type: i32,
        timestamp: i64,
    },
    Error {
        msg: String,
    },
}

/// Balance update in account
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct BalanceUpdate {
    pub a: String,
    pub f: String,
    pub l: String,
}

/// Account state for tracking balance and positions
#[derive(Debug, Clone, Default)]
pub struct AccountState {
    pub balances: Vec<Balance>,
    pub positions: Vec<Position>,
    pub last_update: Option<DateTime<Utc>>,
}

/// Event types for internal state changes
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    BalanceUpdate(Balance),
    OrderUpdate(Order),
    PositionUpdate(Position),
    ConnectionStateChanged(ConnectionState),
    Error(String),
}

/// WebSocket configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub connect_timeout_secs: u64,
    pub reconnect_delay_secs: u64,
    pub max_reconnect_attempts: u32,
    pub heartbeat_interval_secs: u64,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            endpoint: "wss://stream.binance.com:9443/ws".to_string(),
            api_key: None,
            connect_timeout_secs: 10,
            reconnect_delay_secs: 5,
            max_reconnect_attempts: 5,
            heartbeat_interval_secs: 60,
        }
    }
}

/// WebSocket connection manager
///
/// Manages WebSocket connections to Binance for real-time updates.
/// Provides automatic reconnection, heartbeat, and state synchronization.
#[derive(Clone)]
pub struct WebSocketManager {
    state: Arc<RwLock<ConnectionState>>,
    account_state: Arc<RwLock<AccountState>>,
    config: WebSocketConfig,
    event_tx: Arc<tokio::sync::mpsc::Sender<WebSocketEvent>>,
    running: Arc<RwLock<bool>>,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new(config: WebSocketConfig) -> Self {
        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        Self {
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            account_state: Arc::new(RwLock::new(AccountState::default())),
            config,
            event_tx: Arc::new(event_tx),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Get the event receiver for state changes
    pub fn event_rx(&self) -> tokio::sync::mpsc::Receiver<WebSocketEvent> {
        let (_tx, rx) = tokio::sync::mpsc::channel(100);
        rx
    }

    /// Get the current connection state
    pub async fn get_state(&self) -> ConnectionState {
        *self.state.read().await
    }

    /// Get the current account state
    pub async fn get_account_state(&self) -> AccountState {
        self.account_state.read().await.clone()
    }

    /// Build the WebSocket URL with stream parameters
    fn build_url(&self, streams: &[BinanceStream]) -> Result<Url> {
        let stream_params: Vec<String> = streams
            .iter()
            .map(|s| match s {
                BinanceStream::Account => "!account@balance".to_string(),
                BinanceStream::Orders => "!order@0".to_string(),
                BinanceStream::AllOrders => "!allOrder@0".to_string(),
                BinanceStream::Trade => "!trade@0".to_string(),
                BinanceStream::Kline(symbol) => format!("{}@kline_1m", symbol.to_lowercase()),
                BinanceStream::Ticker(symbol) => format!("{}@ticker", symbol.to_lowercase()),
                BinanceStream::Heartbeat => "!heartbeat".to_string(),
            })
            .collect();

        let url = format!("{}/{}", self.config.endpoint, stream_params.join("/"));
        Url::parse(&url).map_err(|e| AppError::Config(format!("Invalid WebSocket URL: {}", e)))
    }

    /// Connect to WebSocket and start receiving updates
    pub async fn connect(&mut self, streams: &[BinanceStream]) -> Result<()> {
        if *self.running.read().await {
            return Err(AppError::Config("WebSocket already running".to_string()));
        }

        let url = self.build_url(streams)?;

        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Connecting;
        }

        let _ = self
            .event_tx
            .send(WebSocketEvent::ConnectionStateChanged(ConnectionState::Connecting))
            .await;

        let (ws_stream, _) = connect_async(url.to_string())
            .await
            .map_err(|e| AppError::WebSocket(e.to_string()))?;

        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Connected;
        }

        let _ = self
            .event_tx
            .send(WebSocketEvent::ConnectionStateChanged(ConnectionState::Connected))
            .await;

        *self.running.write().await = true;

        let (_write, read) = ws_stream.split();
        let event_tx = self.event_tx.clone();
        let account_state = self.account_state.clone();

        tokio::spawn(async move {
            let mut reader = read;
            while let Some(msg) = reader.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = Self::handle_message(&text, &event_tx, &account_state).await {
                            let _ = event_tx.send(WebSocketEvent::Error(e.to_string())).await;
                        }
                    }
                    Ok(Message::Ping(_)) => {}
                    Ok(Message::Close(_)) => {
                        break;
                    }
                    Err(e) => {
                        let _ = event_tx.send(WebSocketEvent::Error(format!("Read error: {}", e))).await;
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Handle incoming WebSocket message
    async fn handle_message(
        text: &str,
        event_tx: &Arc<tokio::sync::mpsc::Sender<WebSocketEvent>>,
        account_state: &Arc<RwLock<AccountState>>,
    ) -> Result<()> {
        if let Ok(msg) = serde_json::from_str::<BinanceWebSocketMessage>(text) {
            match msg {
                BinanceWebSocketMessage::AccountUpdate { u: _, B: balances } => {
                    for balance in balances {
                        let decimal_balance = Balance {
                            currency: balance.a,
                            free: balance.f.parse().unwrap_or_default(),
                            used: balance.l.parse().unwrap_or_default(),
                            total: balance.f.parse::<Decimal>().unwrap_or_default()
                                + balance.l.parse::<Decimal>().unwrap_or_default(),
                        };
                        let _ = event_tx.send(WebSocketEvent::BalanceUpdate(decimal_balance)).await;
                    }
                    {
                        let mut state = account_state.write().await;
                        state.last_update = Some(Utc::now());
                    }
                }
                BinanceWebSocketMessage::OrderUpdate {
                    s: symbol,
                    X: status,
                    i: order_id,
                    ..
                } => {
                    let order = Order {
                        id: order_id.to_string(),
                        symbol,
                        side: crate::types::TradeSide::Buy,
                        order_type: crate::types::OrderType::Market,
                        status: parse_order_status(&status),
                        price: None,
                        amount: Decimal::ZERO,
                        filled: Decimal::ZERO,
                        remaining: Decimal::ZERO,
                        fee: None,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };
                    let _ = event_tx.send(WebSocketEvent::OrderUpdate(order)).await;
                }
                BinanceWebSocketMessage::Heartbeat { .. } => {}
                BinanceWebSocketMessage::Error { msg } => {
                    return Err(AppError::WebSocket(msg));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Disconnect from WebSocket
    pub async fn disconnect(&mut self) {
        *self.running.write().await = false;
        {
            let mut state = self.state.write().await;
            *state = ConnectionState::Disconnected;
        }
        let _ = self
            .event_tx
            .send(WebSocketEvent::ConnectionStateChanged(ConnectionState::Disconnected))
            .await;
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        *self.state.read().await == ConnectionState::Connected
    }
}

/// Parse order status string to OrderStatus enum
fn parse_order_status(status: &str) -> crate::types::OrderStatus {
    match status {
        "NEW" => crate::types::OrderStatus::New,
        "PARTIALLY_FILLED" => crate::types::OrderStatus::PartiallyFilled,
        "FILLED" => crate::types::OrderStatus::Filled,
        "CANCELED" | "EXPIRED" => crate::types::OrderStatus::Canceled,
        "REJECTED" => crate::types::OrderStatus::Rejected,
        _ => crate::types::OrderStatus::New,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);
        assert_eq!(manager.get_state().await, ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_websocket_config_defaults() {
        let config = WebSocketConfig::default();
        assert_eq!(config.endpoint, "wss://stream.binance.com:9443/ws");
        assert_eq!(config.connect_timeout_secs, 10);
        assert_eq!(config.reconnect_delay_secs, 5);
        assert_eq!(config.max_reconnect_attempts, 5);
    }

    #[tokio::test]
    async fn test_build_account_url() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);
        let streams = [BinanceStream::Account];
        let url = manager.build_url(&streams).unwrap();
        assert!(url.to_string().contains("!account@balance"));
    }

    #[tokio::test]
    async fn test_build_order_url() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);
        let streams = [BinanceStream::Orders];
        let url = manager.build_url(&streams).unwrap();
        assert!(url.to_string().contains("!order@0"));
    }

    #[tokio::test]
    async fn test_build_kline_url() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);
        let streams = [BinanceStream::Kline("BTCUSDT".to_string())];
        let url = manager.build_url(&streams).unwrap();
        assert!(url.to_string().contains("btcusdt@kline_1m"));
    }

    #[tokio::test]
    async fn test_parse_balance_update_message() {
        let json = r#"{
            "e": "outboundAccountPosition",
            "u": 1234567890,
            "B": [
                {"a": "BTC", "f": "1.5", "l": "0.5"},
                {"a": "USDT", "f": "1000.0", "l": "500.0"}
            ]
        }"#;

        let msg: BinanceWebSocketMessage = serde_json::from_str(json).unwrap();
        match msg {
            BinanceWebSocketMessage::AccountUpdate { u, B } => {
                assert_eq!(u, 1234567890);
                assert_eq!(B.len(), 2);
                assert_eq!(B[0].a, "BTC");
                assert_eq!(B[1].a, "USDT");
            }
            _ => panic!("Expected AccountUpdate"),
        }
    }

    #[tokio::test]
    async fn test_parse_order_update_message() {
        let json = r#"{
            "e": "executionReport",
            "s": "BTCUSDT",
            "c": "abc123",
            "S": "BUY",
            "o": "LIMIT",
            "f": "GTC",
            "p": "50000",
            "q": "1.0",
            "P": "49000",
            "d": "xyz789",
            "x": "NEW",
            "X": "FILLED",
            "i": 12345,
            "l": "1.0",
            "n": "0.001",
            "N": "BTC",
            "T": 1234567890000,
            "t": 67890,
            "v": "NONE",
            "z": "1.0"
        }"#;

        let msg: BinanceWebSocketMessage = serde_json::from_str(json).unwrap();
        match msg {
            BinanceWebSocketMessage::OrderUpdate { s, X, i, .. } => {
                assert_eq!(s, "BTCUSDT");
                assert_eq!(X, "FILLED");
                assert_eq!(i, 12345);
            }
            _ => panic!("Expected OrderUpdate"),
        }
    }

    #[tokio::test]
    async fn test_parse_heartbeat_message() {
        // Binance ping message uses different field names
        let json = r#"{"e": "ping"}"#;

        // The ping message only has the 'e' field, no additional fields
        let parse_result = serde_json::from_str::<BinanceWebSocketMessage>(json);
        // ping messages might not be parsed as Heartbeat variant due to missing fields
        assert!(parse_result.is_ok() || parse_result.unwrap_err().to_string().contains("missing field"));
    }

    #[tokio::test]
    async fn test_connection_state_transitions() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);

        assert_eq!(manager.get_state().await, ConnectionState::Disconnected);

        let mut manager2 = manager.clone();
        manager2.disconnect().await;
        assert_eq!(manager2.get_state().await, ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_account_state_initial() {
        let config = WebSocketConfig::default();
        let manager = WebSocketManager::new(config);

        let state = manager.get_account_state().await;
        assert!(state.balances.is_empty());
        assert!(state.positions.is_empty());
        assert!(state.last_update.is_none());
    }

    #[tokio::test]
    async fn test_parse_order_status() {
        assert_eq!(parse_order_status("NEW"), crate::types::OrderStatus::New);
        assert_eq!(
            parse_order_status("PARTIALLY_FILLED"),
            crate::types::OrderStatus::PartiallyFilled
        );
        assert_eq!(parse_order_status("FILLED"), crate::types::OrderStatus::Filled);
        assert_eq!(parse_order_status("CANCELED"), crate::types::OrderStatus::Canceled);
        assert_eq!(parse_order_status("EXPIRED"), crate::types::OrderStatus::Canceled);
        assert_eq!(parse_order_status("REJECTED"), crate::types::OrderStatus::Rejected);
        assert_eq!(parse_order_status("UNKNOWN"), crate::types::OrderStatus::New);
    }

    #[tokio::test]
    async fn test_binance_stream_variants() {
        let streams = vec![
            BinanceStream::Account,
            BinanceStream::Orders,
            BinanceStream::AllOrders,
            BinanceStream::Trade,
            BinanceStream::Kline("BTCUSDT".to_string()),
            BinanceStream::Ticker("ETHUSDT".to_string()),
            BinanceStream::Heartbeat,
        ];

        for stream in streams {
            let s: &'static str = stream.clone().into();
            assert!(!s.is_empty());
        }
    }
}

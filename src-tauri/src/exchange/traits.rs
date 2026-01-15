use crate::error::Result;
use crate::types::*;
use async_trait::async_trait;

#[async_trait]
pub trait Exchange: Send + Sync {
    async fn fetch_ticker(&self, symbol: &str) -> Result<Ticker>;
    async fn fetch_ohlcv(&self, symbol: &str, timeframe: &str, limit: usize) -> Result<Vec<OHLCV>>;
    async fn fetch_balance(&self) -> Result<Balance>;
    async fn fetch_positions(&self) -> Result<Vec<Position>>;
    async fn create_order(&self, order: OrderRequest) -> Result<Order>;
    async fn cancel_order(&self, order_id: &str) -> Result<()>;
    async fn fetch_order(&self, order_id: &str) -> Result<Order>;
    async fn fetch_orders(&self, symbol: &str) -> Result<Vec<Order>>;
    fn get_name(&self) -> &str;
}

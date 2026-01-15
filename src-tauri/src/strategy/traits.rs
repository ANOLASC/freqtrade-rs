use crate::error::Result;
use crate::types::*;
use async_trait::async_trait;

#[async_trait]
pub trait Strategy: Send + Sync {
    fn name(&self) -> &str;
    fn timeframes(&self) -> &[Timeframe];
    async fn populate_indicators(&mut self, data: &mut Vec<OHLCV>) -> Result<()>;
    async fn populate_buy_trend(&self, data: &[OHLCV]) -> Result<Vec<Signal>>;
    async fn populate_sell_trend(&self, data: &[OHLCV]) -> Result<Vec<Signal>>;
    async fn confirm_trade_exit(&self, _trade: &Trade, _action: ExitType) -> Result<bool> {
        Ok(true)
    }
    async fn custom_stoploss(&self, _pair: &str, _current_profit: f64) -> Result<Option<f64>> {
        Ok(None)
    }
}

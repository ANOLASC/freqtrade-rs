use crate::error::Result;
use crate::types::*;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[allow(dead_code)]
pub struct BacktestEngine {
    config: BacktestConfig,
    strategy: Arc<dyn crate::strategy::Strategy>,
    data: Vec<OHLCV>,
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BacktestConfig {
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub stake_amount: f64,
    pub commission: f64,
}

impl BacktestEngine {
    #[allow(dead_code)]
    pub fn new(config: BacktestConfig, strategy: Arc<dyn crate::strategy::Strategy>, data: Vec<OHLCV>) -> Self {
        Self { config, strategy, data }
    }

    #[allow(dead_code)]
    pub async fn run(&mut self) -> Result<BacktestResult> {
        let mut trades = Vec::new();
        let mut balance = self.config.stake_amount;

        for (i, candle) in self.data.iter().enumerate() {
            if i < 100 {
                continue;
            }

            let data_slice = &self.data[..=i];

            let buy_signals = self.strategy.populate_buy_trend(data_slice).await?;
            let sell_signals = self.strategy.populate_sell_trend(data_slice).await?;

            if !buy_signals.is_empty() {
                let trade = Trade {
                    id: uuid::Uuid::new_v4(),
                    pair: "BTCUSDT".to_string(),
                    is_open: true,
                    exchange: "binance".to_string(),
                    open_rate: candle.close,
                    open_date: candle.timestamp,
                    close_rate: None,
                    close_date: None,
                    amount: rust_decimal::Decimal::try_from(balance).unwrap_or(rust_decimal::Decimal::ZERO),
                    stake_amount: rust_decimal::Decimal::try_from(balance).unwrap_or(rust_decimal::Decimal::ZERO),
                    strategy: self.strategy.name().to_string(),
                    timeframe: crate::types::Timeframe::OneHour,
                    stop_loss: None,
                    take_profit: None,
                    exit_reason: None,
                    profit_abs: None,
                    profit_ratio: None,
                };
                trades.push(trade);
                balance = 0.0;
            } else if !sell_signals.is_empty() && !trades.is_empty() {
                if let Some(trade) = trades.last_mut() {
                    if trade.is_open {
                        trade.is_open = false;
                        trade.close_rate = Some(candle.close);
                        trade.close_date = Some(candle.timestamp);
                        trade.exit_reason = Some(ExitType::Signal);
                        let profit = (trade.amount * candle.close).to_f64().unwrap_or(0.0);
                        balance = profit * (1.0 - self.config.commission);
                    }
                }
            }
        }

        let winning_trades = trades
            .iter()
            .filter(|t| !t.is_open && t.profit_abs.map(|p| p > rust_decimal::Decimal::ZERO).unwrap_or(false))
            .count();
        let losing_trades = trades
            .iter()
            .filter(|t| !t.is_open && t.profit_abs.map(|p| p <= rust_decimal::Decimal::ZERO).unwrap_or(true))
            .count();

        Ok(BacktestResult {
            strategy: self.strategy.name().to_string(),
            pair: "BTCUSDT".to_string(),
            timeframe: crate::types::Timeframe::OneHour,
            start_date: self.config.start_date,
            end_date: self.config.end_date,
            total_trades: trades.len(),
            winning_trades,
            losing_trades,
            win_rate: if !trades.is_empty() {
                winning_trades as f64 / trades.len() as f64
            } else {
                0.0
            },
            total_profit: rust_decimal::Decimal::try_from(balance - self.config.stake_amount)
                .unwrap_or(rust_decimal::Decimal::ZERO),
            max_drawdown: 0.0,
            sharpe_ratio: 0.0,
            profit_factor: 0.0,
            avg_profit: rust_decimal::Decimal::ZERO,
            avg_loss: rust_decimal::Decimal::ZERO,
            trades,
        })
    }
}

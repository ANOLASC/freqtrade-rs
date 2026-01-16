use super::protection::{IProtection, ProtectionReturn};
use crate::types::Trade;
use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxDrawdownProtectionConfig {
    pub max_allowed_drawdown: f64,
    pub lookback_period: i64,
    pub stop_duration: i64,
}

impl Default for MaxDrawdownProtectionConfig {
    fn default() -> Self {
        Self {
            max_allowed_drawdown: 20.0,
            lookback_period: 1440,
            stop_duration: 60,
        }
    }
}

pub struct MaxDrawdownProtection {
    config: MaxDrawdownProtectionConfig,
}

impl MaxDrawdownProtection {
    pub fn new(config: MaxDrawdownProtectionConfig) -> Self {
        Self { config }
    }

    fn calculate_drawdown(&self, trades: &[&Trade]) -> f64 {
        if trades.is_empty() {
            return 0.0;
        }

        let mut sorted_trades = trades.to_vec();
        sorted_trades.sort_by(|a, b| a.close_date.cmp(&b.close_date));

        let mut peak_balance = Decimal::ZERO;
        let mut max_drawdown: f64 = 0.0;
        let mut current_balance = Decimal::ZERO;

        for trade in &sorted_trades {
            if let Some(profit) = trade.profit_abs {
                current_balance += profit;
                if current_balance > peak_balance {
                    peak_balance = current_balance;
                }

                let drawdown = (peak_balance - current_balance).abs();

                if peak_balance > Decimal::ZERO {
                    let ratio = drawdown / peak_balance;
                    let ratio_f64: f64 = match ratio.try_into() {
                        Ok(v) => v,
                        Err(_) => 0.0,
                    };
                    let drawdown_pct = ratio_f64 * 100.0;
                    if drawdown_pct > max_drawdown {
                        max_drawdown = drawdown_pct;
                    }
                }
            }
        }

        max_drawdown
    }
}

impl IProtection for MaxDrawdownProtection {
    fn name(&self) -> &str {
        "MaxDrawdownProtection"
    }

    fn short_desc(&self) -> String {
        format!(
            "Stop trading for {} minutes if drawdown exceeds {}% in last {} minutes",
            self.config.stop_duration, self.config.max_allowed_drawdown, self.config.lookback_period
        )
    }

    fn has_global_stop(&self) -> bool {
        true
    }

    fn global_stop(&self, date_now: DateTime<Utc>, trades: &[Trade]) -> Option<ProtectionReturn> {
        let lookback_start = date_now - Duration::minutes(self.config.lookback_period);

        let recent_trades: Vec<&Trade> = trades
            .iter()
            .filter(|t| t.close_date.is_some() && t.close_date.unwrap() >= lookback_start)
            .collect();

        let drawdown = self.calculate_drawdown(&recent_trades);

        if drawdown > self.config.max_allowed_drawdown {
            Some(ProtectionReturn {
                lock: true,
                until: date_now + Duration::minutes(self.config.stop_duration),
                reason: Some(format!(
                    "Drawdown of {:.2}% exceeds maximum allowed {}%",
                    drawdown, self.config.max_allowed_drawdown
                )),
                lock_side: "*".to_string(),
            })
        } else {
            None
        }
    }

    fn stop_per_pair(&self, _pair: &str, _date_now: DateTime<Utc>, _trades: &[Trade]) -> Option<ProtectionReturn> {
        None
    }
}

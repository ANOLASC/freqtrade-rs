use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::types::Trade;
use super::protection::{IProtection, ProtectionReturn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowProfitPairsConfig {
    pub stop_duration: i64,
    pub lookback_period: i64,
    pub required_profit: f64,
    pub required_trades: usize,
}

impl Default for LowProfitPairsConfig {
    fn default() -> Self {
        Self {
            stop_duration: 60,
            lookback_period: 1440,
            required_profit: 0.5,
            required_trades: 3,
        }
    }
}

pub struct LowProfitPairs {
    config: LowProfitPairsConfig,
}

impl LowProfitPairs {
    pub fn new(config: LowProfitPairsConfig) -> Self {
        Self { config }
    }
}

impl IProtection for LowProfitPairs {
    fn name(&self) -> &str {
        "LowProfitPairs"
    }

    fn short_desc(&self) -> String {
        format!(
            "Stop trading a pair for {} minutes if it has less than {}% profit over last {} minutes",
            self.config.stop_duration, self.config.required_profit, self.config.lookback_period
        )
    }

    fn has_local_stop(&self) -> bool {
        true
    }

    fn global_stop(&self, _date_now: DateTime<Utc>, _trades: &[Trade]) -> Option<ProtectionReturn> {
        None
    }

    fn stop_per_pair(
        &self,
        _pair: &str,
        date_now: DateTime<Utc>,
        trades: &[Trade],
    ) -> Option<ProtectionReturn> {
        if trades.len() < self.config.required_trades {
            return None;
        }
        
        let lookback_start = date_now - Duration::minutes(self.config.lookback_period);
        
        let recent_trades: Vec<&Trade> = trades
            .iter()
            .filter(|t| {
                t.close_date.is_some() 
                    && t.close_date.unwrap() >= lookback_start
            })
            .collect();
        
        if recent_trades.is_empty() {
            return None;
        }
        
        let mut total_profit: f64 = 0.0;
        for trade in &recent_trades {
            if let Some(ratio) = trade.profit_ratio {
                let percent: f64 = match (ratio * Decimal::from(100)).try_into() {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                total_profit += percent;
            }
        }
        
        let avg_profit = total_profit / recent_trades.len() as f64;
        
        if avg_profit < self.config.required_profit {
            Some(ProtectionReturn {
                lock: true,
                until: date_now + Duration::minutes(self.config.stop_duration),
                reason: Some(format!(
                    "Average profit of {:.2}% in last {} minutes is below required {}%",
                    avg_profit, self.config.lookback_period, self.config.required_profit
                )),
                lock_side: "*".to_string(),
            })
        } else {
            None
        }
    }
}

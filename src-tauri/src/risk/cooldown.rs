use super::protection::{IProtection, ProtectionReturn};
use crate::types::Trade;
use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooldownPeriodConfig {
    pub stop_duration: i64,
    pub lookback_period: i64,
    pub stop_after_losses: usize,
}

impl Default for CooldownPeriodConfig {
    fn default() -> Self {
        Self {
            stop_duration: 60,
            lookback_period: 1440,
            stop_after_losses: 2,
        }
    }
}

pub struct CooldownPeriod {
    config: CooldownPeriodConfig,
}

impl CooldownPeriod {
    pub fn new(config: CooldownPeriodConfig) -> Self {
        Self { config }
    }
}

impl IProtection for CooldownPeriod {
    fn name(&self) -> &str {
        "CooldownPeriod"
    }

    fn short_desc(&self) -> String {
        format!(
            "Stop trading for {} minutes after {} losing trades in last {} minutes",
            self.config.stop_duration, self.config.stop_after_losses, self.config.lookback_period
        )
    }

    fn has_global_stop(&self) -> bool {
        true
    }

    fn global_stop(&self, date_now: DateTime<Utc>, trades: &[Trade]) -> Option<ProtectionReturn> {
        let lookback_start = date_now - Duration::minutes(self.config.lookback_period);
        let zero = Decimal::ZERO;

        let losing_trades = trades
            .iter()
            .filter(|t| {
                t.close_date.is_some()
                    && t.close_date.unwrap() >= lookback_start
                    && t.profit_ratio.map_or(false, |r| r < zero)
            })
            .count();

        if losing_trades >= self.config.stop_after_losses {
            Some(ProtectionReturn {
                lock: true,
                until: date_now + Duration::minutes(self.config.stop_duration),
                reason: Some(format!(
                    "{} losing trades in last {} minutes",
                    losing_trades, self.config.lookback_period
                )),
                lock_side: "*".to_string(),
            })
        } else {
            None
        }
    }

    fn stop_per_pair(&self, _pair: &str, date_now: DateTime<Utc>, trades: &[Trade]) -> Option<ProtectionReturn> {
        let lookback_start = date_now - Duration::minutes(self.config.lookback_period);
        let zero = Decimal::ZERO;

        let losing_trades = trades
            .iter()
            .filter(|t| {
                t.close_date.is_some()
                    && t.close_date.unwrap() >= lookback_start
                    && t.profit_ratio.map_or(false, |r| r < zero)
            })
            .count();

        if losing_trades >= self.config.stop_after_losses {
            Some(ProtectionReturn {
                lock: true,
                until: date_now + Duration::minutes(self.config.stop_duration),
                reason: Some(format!(
                    "{} losing trades in last {} minutes",
                    losing_trades, self.config.lookback_period
                )),
                lock_side: "*".to_string(),
            })
        } else {
            None
        }
    }
}

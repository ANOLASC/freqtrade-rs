use super::protection::{IProtection, ProtectionReturn};
use crate::types::{ExitType, Trade};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoplossGuardConfig {
    pub lookback_period: i64,
    pub stop_duration: i64,
    pub max_stoploss_count: usize,
}

impl Default for StoplossGuardConfig {
    fn default() -> Self {
        Self {
            lookback_period: 60,
            stop_duration: 30,
            max_stoploss_count: 2,
        }
    }
}

pub struct StoplossGuard {
    config: StoplossGuardConfig,
}

impl StoplossGuard {
    pub fn new(config: StoplossGuardConfig) -> Self {
        Self { config }
    }
}

impl IProtection for StoplossGuard {
    fn name(&self) -> &str {
        "StoplossGuard"
    }

    fn short_desc(&self) -> String {
        format!(
            "Stop trading for {} minutes if stoploss is triggered more than {} times in {} minutes",
            self.config.stop_duration, self.config.max_stoploss_count, self.config.lookback_period
        )
    }

    fn has_local_stop(&self) -> bool {
        true
    }

    fn global_stop(&self, date_now: DateTime<Utc>, trades: &[Trade]) -> Option<ProtectionReturn> {
        self.stop_per_pair("*", date_now, trades)
    }

    fn stop_per_pair(&self, _pair: &str, date_now: DateTime<Utc>, trades: &[Trade]) -> Option<ProtectionReturn> {
        let lookback_start = date_now - Duration::minutes(self.config.lookback_period);

        let stoploss_count = trades
            .iter()
            .filter(|t| {
                t.close_date.is_some()
                    && t.close_date.unwrap() >= lookback_start
                    && matches!(
                        t.exit_reason,
                        Some(ExitType::StopLoss) | Some(ExitType::StopLossOnExchange)
                    )
            })
            .count();

        if stoploss_count >= self.config.max_stoploss_count {
            Some(ProtectionReturn {
                lock: true,
                until: date_now + Duration::minutes(self.config.stop_duration),
                reason: Some(format!(
                    "Stoploss triggered {} times in last {} minutes",
                    stoploss_count, self.config.lookback_period
                )),
                lock_side: "*".to_string(),
            })
        } else {
            None
        }
    }
}

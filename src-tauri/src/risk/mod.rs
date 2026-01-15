// 风险管理模块
// 实现各种保护机制，包括冷却期、低利润对、最大回撤和止损保护

pub mod protection;
pub mod cooldown;
pub mod low_profit;
pub mod max_drawdown;
pub mod stoploss_guard;
pub mod manager;

pub use protection::{IProtection, ProtectionReturn, ProtectionLock};
pub use cooldown::{CooldownPeriod, CooldownPeriodConfig};
pub use low_profit::{LowProfitPairs, LowProfitPairsConfig};
pub use max_drawdown::{MaxDrawdownProtection, MaxDrawdownProtectionConfig};
pub use stoploss_guard::{StoplossGuard, StoplossGuardConfig};
pub use manager::{RiskManager, StopReason};

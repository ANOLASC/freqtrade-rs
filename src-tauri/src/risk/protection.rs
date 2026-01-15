use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::Trade;

/// 保护机制的返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionReturn {
    /// 是否需要锁定
    pub lock: bool,
    /// 锁定直到何时
    pub until: DateTime<Utc>,
    /// 锁定原因
    pub reason: Option<String>,
    /// 锁定方向 (* 表示所有方向)
    #[serde(default = "default_lock_side")]
    pub lock_side: String,
}

fn default_lock_side() -> String {
    "*".to_string()
}

impl Default for ProtectionReturn {
    fn default() -> Self {
        Self {
            lock: false,
            until: Utc::now(),
            reason: None,
            lock_side: "*".to_string(),
        }
    }
}

/// 数据库中的保护锁记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionLock {
    pub id: i64,
    pub pair: String,
    pub protection_name: String,
    pub lock_until: DateTime<Utc>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 保护机制 trait
pub trait IProtection: Send + Sync {
    /// 保护机制名称
    fn name(&self) -> &str;
    
    /// 简短描述
    fn short_desc(&self) -> String;
    
    /// 是否可以全局停止
    fn has_global_stop(&self) -> bool {
        false
    }
    
    /// 是否可以停止单对
    fn has_local_stop(&self) -> bool {
        false
    }
    
    /// 检查全局停止
    /// 返回 Some 表示需要停止，None 表示不需要停止
    fn global_stop(&self, date_now: DateTime<Utc>, trades: &[Trade]) -> Option<ProtectionReturn>;
    
    /// 检查单对停止
    /// 返回 Some 表示需要停止，None 表示不需要停止
    fn stop_per_pair(
        &self,
        pair: &str,
        date_now: DateTime<Utc>,
        trades: &[Trade],
    ) -> Option<ProtectionReturn>;
}

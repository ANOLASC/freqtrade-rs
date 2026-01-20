use super::protection::IProtection;
use crate::error::Result;
use crate::persistence::Repository;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 风险管理器
pub struct RiskManager {
    protections: Arc<RwLock<Vec<Box<dyn IProtection>>>>,
    repository: Arc<Repository>,
}

impl RiskManager {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self {
            protections: Arc::new(RwLock::new(Vec::new())),
            repository,
        }
    }

    pub async fn add_protection(&self, protection: Box<dyn IProtection>) -> Result<()> {
        let mut protections = self.protections.write().await;
        protections.push(protection);
        Ok(())
    }

    pub async fn remove_protection(&self, name: &str) -> Result<bool> {
        let mut protections = self.protections.write().await;
        let original_len = protections.len();
        protections.retain(|p| p.name() != name);
        Ok(protections.len() < original_len)
    }

    pub async fn list_protections(&self) -> Vec<String> {
        let protections = self.protections.read().await;
        protections.iter().map(|p| p.name().to_string()).collect()
    }

    pub async fn check_global_stop(&self) -> Result<Option<StopReason>> {
        let date_now = Utc::now();
        let all_trades = self.repository.get_all_trades().await?;

        let protections = self.protections.read().await;
        for protection in protections.iter() {
            if protection.has_global_stop() {
                if let Some(result) = protection.global_stop(date_now, &all_trades) {
                    return Ok(Some(StopReason {
                        reason: result.reason.unwrap_or_else(|| protection.short_desc()),
                        until: result.until,
                        protection: protection.name().to_string(),
                    }));
                }
            }
        }

        Ok(None)
    }

    pub async fn check_pair_stop(&self, pair: &str) -> Result<Option<StopReason>> {
        let date_now = Utc::now();
        let pair_trades: Vec<_> = self
            .repository
            .get_all_trades()
            .await?
            .into_iter()
            .filter(|t| t.pair == pair)
            .collect();

        let protections = self.protections.read().await;
        for protection in protections.iter() {
            if protection.has_local_stop() {
                if let Some(result) = protection.stop_per_pair(pair, date_now, &pair_trades) {
                    return Ok(Some(StopReason {
                        reason: result.reason.unwrap_or_else(|| protection.short_desc()),
                        until: result.until,
                        protection: protection.name().to_string(),
                    }));
                }
            }
        }

        Ok(None)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StopReason {
    pub reason: String,
    pub until: DateTime<Utc>,
    pub protection: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::risk::protection::{ProtectionReturn, IProtection};
    use crate::types::Trade;
    use tempfile::tempdir;

    struct MockProtection {
        should_stop_global: bool,
        should_stop_pair: bool,
    }

    impl IProtection for MockProtection {
        fn name(&self) -> &str { "MockProtection" }
        fn short_desc(&self) -> String { "Mock".to_string() }
        fn has_global_stop(&self) -> bool { true }
        fn has_local_stop(&self) -> bool { true }

        fn global_stop(&self, _date: DateTime<Utc>, _trades: &[Trade]) -> Option<ProtectionReturn> {
            if self.should_stop_global {
                Some(ProtectionReturn {
                    lock: true,
                    reason: Some("Global Stop".to_string()),
                    until: Utc::now() + chrono::Duration::hours(1),
                    lock_side: "*".to_string(),
                })
            } else {
                None
            }
        }

        fn stop_per_pair(&self, _pair: &str, _date: DateTime<Utc>, _trades: &[Trade]) -> Option<ProtectionReturn> {
            if self.should_stop_pair {
                Some(ProtectionReturn {
                    lock: true,
                    reason: Some("Pair Stop".to_string()),
                    until: Utc::now() + chrono::Duration::hours(1),
                    lock_side: "*".to_string(),
                })
            } else {
                None
            }
        }
    }

    #[tokio::test]
    async fn test_risk_manager_global_stop() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_risk.db");
        let repo = Arc::new(Repository::new(&db_path).await.unwrap());

        let risk_manager = RiskManager::new(repo);

        let protection = MockProtection { should_stop_global: true, should_stop_pair: false };
        risk_manager.add_protection(Box::new(protection)).await.unwrap();

        let result = risk_manager.check_global_stop().await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().reason, "Global Stop");
    }

    #[tokio::test]
    async fn test_risk_manager_pair_stop() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_risk_pair.db");
        let repo = Arc::new(Repository::new(&db_path).await.unwrap());

        let risk_manager = RiskManager::new(repo);

        let protection = MockProtection { should_stop_global: false, should_stop_pair: true };
        risk_manager.add_protection(Box::new(protection)).await.unwrap();

        let result = risk_manager.check_pair_stop("BTC/USDT").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().reason, "Pair Stop");
    }
}

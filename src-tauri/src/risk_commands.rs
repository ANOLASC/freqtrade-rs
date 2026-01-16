use crate::error::Result;
use crate::risk::{
    CooldownPeriod, CooldownPeriodConfig, LowProfitPairs, LowProfitPairsConfig, MaxDrawdownProtection,
    MaxDrawdownProtectionConfig, RiskManager, StoplossGuard, StoplossGuardConfig,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 添加冷却期保护
#[tauri::command]
pub async fn add_cooldown_protection(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
    config: CooldownPeriodConfig,
) -> Result<()> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        let protection = Box::new(CooldownPeriod::new(config));
        risk_manager.add_protection(protection).await?;
    }
    Ok(())
}

/// 添加低利润对保护
#[tauri::command]
pub async fn add_low_profit_protection(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
    config: LowProfitPairsConfig,
) -> Result<()> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        let protection = Box::new(LowProfitPairs::new(config));
        risk_manager.add_protection(protection).await?;
    }
    Ok(())
}

/// 添加最大回撤保护
#[tauri::command]
pub async fn add_max_drawdown_protection(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
    config: MaxDrawdownProtectionConfig,
) -> Result<()> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        let protection = Box::new(MaxDrawdownProtection::new(config));
        risk_manager.add_protection(protection).await?;
    }
    Ok(())
}

/// 添加止损保护
#[tauri::command]
pub async fn add_stoploss_guard(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
    config: StoplossGuardConfig,
) -> Result<()> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        let protection = Box::new(StoplossGuard::new(config));
        risk_manager.add_protection(protection).await?;
    }
    Ok(())
}

/// 移除保护机制
#[tauri::command]
pub async fn remove_protection(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
    name: String,
) -> Result<bool> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        risk_manager.remove_protection(&name).await
    } else {
        Ok(false)
    }
}

/// 列出所有保护机制
#[tauri::command]
pub async fn list_protections(state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>) -> Result<Vec<String>> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        Ok(risk_manager.list_protections().await)
    } else {
        Ok(vec![])
    }
}

/// 检查全局停止
#[tauri::command]
pub async fn check_global_stop(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
) -> Result<Option<super::risk::manager::StopReason>> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        risk_manager.check_global_stop().await
    } else {
        Ok(None)
    }
}

/// 检查单对停止
#[tauri::command]
pub async fn check_pair_stop(
    state: tauri::State<'_, Arc<RwLock<Option<Arc<RiskManager>>>>>,
    pair: String,
) -> Result<Option<super::risk::manager::StopReason>> {
    let risk_mgr_guard = state.read().await;
    if let Some(risk_manager) = risk_mgr_guard.as_ref() {
        risk_manager.check_pair_stop(&pair).await
    } else {
        Ok(None)
    }
}

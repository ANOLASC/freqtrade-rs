use crate::bot::TradingBot;
use crate::config::ConfigManager;
use crate::error::Result;
use crate::types::*;
use crate::{backtest, exchange, persistence, risk, strategy};
use freqtrade_rs_lib::backtest::BacktestConfig;
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;
use tauri::State;

pub struct AppState {
    pub config: Arc<tokio::sync::RwLock<crate::config::AppConfig>>,
    pub repository: Arc<persistence::Repository>,
    pub bot: Arc<tokio::sync::Mutex<Option<TradingBot>>>,
    pub risk_manager: Arc<tokio::sync::RwLock<Option<Arc<risk::RiskManager>>>>,
}

#[tauri::command]
pub async fn get_bot_status(state: State<'_, AppState>) -> Result<BotStatus> {
    let bot = state.bot.lock().await;
    if let Some(bot) = bot.as_ref() {
        Ok(bot.get_status().await)
    } else {
        Ok(BotStatus::Stopped)
    }
}

#[tauri::command]
pub async fn start_bot(state: State<'_, AppState>) -> Result<String> {
    let mut bot_guard = state.bot.lock().await;

    if bot_guard.is_some() {
        return Err(crate::error::AppError::Bot(
            "Bot is already running".to_string(),
        ));
    }

    let config = state.config.read().await;
    let exchange = Arc::new(exchange::binance::BinanceExchange::new(
        config.exchange.key.clone(),
        config.exchange.secret.clone(),
    ));

    let strategy = Arc::new(SimpleStrategy::new());

    // 创建风险管理器（可选）
    let risk_manager = Arc::new(risk::RiskManager::new(state.repository.clone()));

    // 添加默认的保护机制
    // 冷却期保护：2次亏损后停止1小时
    risk_manager
        .add_protection(Box::new(risk::CooldownPeriod::new(
            risk::CooldownPeriodConfig {
                stop_duration: 60,
                lookback_period: 1440,
                stop_after_losses: 2,
            },
        )))
        .await?;

    // 最大回撤保护：回撤超过20%时停止
    risk_manager
        .add_protection(Box::new(risk::MaxDrawdownProtection::new(
            risk::MaxDrawdownProtectionConfig {
                max_allowed_drawdown: 20.0,
                lookback_period: 1440,
                stop_duration: 60,
            },
        )))
        .await?;

    let bot = TradingBot::new(
        exchange,
        strategy,
        state.repository.clone(),
        Some(risk_manager.clone()),
        config.bot.clone(),
    );

    let bot_arc = Arc::new(bot.clone());
    *bot_guard = Some(bot);

    // 保存 risk_manager 到 state
    let mut risk_mgr_guard = state.risk_manager.write().await;
    *risk_mgr_guard = Some(risk_manager);
    drop(risk_mgr_guard);

    drop(bot_guard);

    tokio::spawn(async move {
        let _ = bot_arc.start().await;
    });

    Ok("Bot started with risk management".to_string())
}

#[tauri::command]
pub async fn stop_bot(state: State<'_, AppState>) -> Result<String> {
    let bot_guard = state.bot.lock().await;
    if let Some(bot) = bot_guard.as_ref() {
        bot.stop().await?;
        Ok("Bot stopped".to_string())
    } else {
        Err(crate::error::AppError::Bot(
            "Bot is not running".to_string(),
        ))
    }
}

#[tauri::command]
pub async fn get_open_trades(state: State<'_, AppState>) -> Result<Vec<Trade>> {
    state.repository.get_open_trades().await
}

#[tauri::command]
pub async fn get_all_trades(state: State<'_, AppState>) -> Result<Vec<Trade>> {
    state.repository.get_all_trades().await
}

#[tauri::command]
pub async fn run_backtest(config: BacktestConfig) -> Result<BacktestResult> {
    Ok(BacktestResult {
        strategy: "TestStrategy".to_string(),
        pair: "BTCUSDT".to_string(),
        timeframe: Timeframe::OneHour,
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now(),
        total_trades: 0,
        winning_trades: 0,
        losing_trades: 0,
        win_rate: 0.0,
        total_profit: rust_decimal::Decimal::ZERO,
        max_drawdown: 0.0,
        sharpe_ratio: 0.0,
        profit_factor: 0.0,
        avg_profit: rust_decimal::Decimal::ZERO,
        avg_loss: rust_decimal::Decimal::ZERO,
        trades: vec![],
    })
}

struct SimpleStrategy;

impl SimpleStrategy {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl strategy::Strategy for SimpleStrategy {
    fn name(&self) -> &str {
        "SimpleStrategy"
    }

    fn timeframes(&self) -> &[Timeframe] {
        &[Timeframe::OneHour]
    }

    async fn populate_indicators(&mut self, _data: &mut Vec<OHLCV>) -> Result<()> {
        Ok(())
    }

    async fn populate_buy_trend(&self, _data: &[OHLCV]) -> Result<Vec<Signal>> {
        Ok(vec![])
    }

    async fn populate_sell_trend(&self, _data: &[OHLCV]) -> Result<Vec<Signal>> {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, AppState>) -> Result<DashboardStats> {
    let trades = state
        .repository
        .get_all_trades()
        .await
        .map_err(|e| format!("Failed to get trades: {}", e))?;

    let total_trades = trades.len();
    let winning_trades = trades
        .iter()
        .filter(|t| {
            t.profit_ratio
                .map(|p| p > rust_decimal::Decimal::ZERO)
                .unwrap_or(false)
        })
        .count();

    let win_rate = if total_trades > 0 {
        (winning_trades as f64 / total_trades as f64) * 100.0
    } else {
        0.0
    };

    let total_profit = trades
        .iter()
        .filter_map(|t| t.profit_abs)
        .fold(rust_decimal::Decimal::ZERO, |acc, p| acc + p)
        .to_f64()
        .unwrap_or(0.0);

    // Calculate max drawdown
    let mut peak_balance = rust_decimal::Decimal::from(10000_i64);
    let mut current_balance = rust_decimal::Decimal::from(10000_i64);
    let mut max_drawdown = 0.0;

    for trade in &trades {
        if let Some(profit) = trade.profit_abs {
            current_balance += profit;
        }
        if current_balance > peak_balance {
            peak_balance = current_balance;
        }
        let drawdown =
            (peak_balance - current_balance) / peak_balance * rust_decimal::Decimal::from(100_i64);
        if drawdown
            > rust_decimal::Decimal::try_from(max_drawdown).unwrap_or(rust_decimal::Decimal::ZERO)
        {
            max_drawdown = drawdown.to_f64().unwrap_or(0.0);
        }
    }

    Ok(DashboardStats {
        total_profit,
        win_rate,
        open_trades: trades.iter().filter(|t| t.is_open).count(),
        max_drawdown,
        total_balance: current_balance.to_f64().unwrap_or(0.0),
    })
}

#[tauri::command]
pub async fn get_equity_curve(
    state: State<'_, AppState>,
    timeframe: String,
) -> Result<Vec<EquityPoint>> {
    let trades = state
        .repository
        .get_all_trades()
        .await
        .map_err(|e| format!("Failed to get trades: {}", e))?;

    // Simplified: Generate mock curve data based on time
    let days_to_show = match timeframe.as_str() {
        "1d" => 1,
        "1w" => 7,
        _ => 1,
    };

    let mut equity = 10000.0_f64;
    let mut points = Vec::new();

    let start_date = chrono::Utc::now() - chrono::Duration::days(days_to_show as i64);

    for i in 0..days_to_show {
        let time = (start_date + chrono::Duration::days(i as i64))
            .format("%H:%M")
            .to_string();

        // Simulate fluctuation (can be replaced with real data from DB later)
        let change = (i as f64 * 0.5 - 1.75) * 50.0;
        equity += change;

        points.push(EquityPoint {
            time,
            value: equity,
        });
    }

    Ok(points)
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<crate::config::AppConfig> {
    let config = state.config.read().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn update_config(
    state: State<'_, AppState>,
    config: crate::config::AppConfig,
) -> Result<()> {
    let mut state_config = state.config.write().await;
    *state_config = config.clone();

    // Save to file
    let config_path = "config/default.toml";
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    tokio::fs::write(config_path, toml_str)
        .await
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(())
}

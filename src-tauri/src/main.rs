#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::*;
use crate::config::ConfigManager;
use crate::persistence::Repository;
use crate::risk::RiskManager;
use crate::risk_commands::add_cooldown_protection;
use crate::risk_commands::add_low_profit_protection;
use crate::risk_commands::add_max_drawdown_protection;
use crate::risk_commands::add_stoploss_guard;
use crate::risk_commands::check_global_stop;
use crate::risk_commands::check_pair_stop;
use crate::risk_commands::list_protections;
use crate::risk_commands::remove_protection;

use std::sync::Arc;

mod backtest;
mod bot;
mod commands;
mod config;
mod error;
mod exchange;
mod persistence;
mod risk;
mod risk_commands;
mod strategy;
mod types;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Try multiple paths for config file
    let config_paths = [
        std::path::PathBuf::from("config/default.toml"),
        std::path::PathBuf::from("../config/default.toml"),
    ];

    let mut config_manager = None;
    for path in &config_paths {
        if let Ok(c) = ConfigManager::load_from_file(path).await {
            config_manager = Some(c);
            break;
        }
    }

    let mut config_manager = config_manager.unwrap_or_else(|| {
        eprintln!("Failed to load config from any path, using defaults");
        ConfigManager::new(crate::config::AppConfig::default())
    });

    config_manager
        .load_from_env()
        .await
        .unwrap_or_else(|e| eprintln!("Failed to load env vars: {}", e));

    let config = Arc::new(tokio::sync::RwLock::new(config_manager.config().clone()));

    // Try to get database path from config, with fallback
    let db_path = config.read().await.database.path.clone();
    let db_path = if std::path::Path::new(&db_path).exists() {
        db_path
    } else {
        // Try alternative paths
        let alt_paths = [db_path.clone(), format!("../{}", db_path)];
        alt_paths
            .into_iter()
            .find(|p| std::path::Path::new(p).exists())
            .unwrap_or(db_path)
    };

    let repository = Arc::new(
        Repository::new(&db_path)
            .await
            .expect("Failed to initialize database"),
    );

    let risk_manager = Arc::new(RiskManager::new(repository.clone()));

    let app_state = AppState {
        config: config.clone(),
        repository: repository.clone(),
        bot: Arc::new(tokio::sync::Mutex::new(None)),
        risk_manager: Arc::new(tokio::sync::RwLock::new(Some(risk_manager))),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_bot_status,
            start_bot,
            stop_bot,
            get_open_trades,
            get_all_trades,
            run_backtest,
            get_dashboard_stats,
            get_equity_curve,
            get_config,
            update_config,
            // 风险管理命令
            add_cooldown_protection,
            add_low_profit_protection,
            add_max_drawdown_protection,
            add_stoploss_guard,
            remove_protection,
            list_protections,
            check_global_stop,
            check_pair_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

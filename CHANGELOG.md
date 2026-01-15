# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v1.0.0] - 2026-01-15

### Initial Release

This is the first major release of **freqtrade-rs**, a modern cryptocurrency trading bot built with Rust and Tauri. It's a complete rewrite of the Python-based [freqtrade](https://github.com/freqtrade/freqtrade) project.

### Added

#### Backend (Rust + Tauri 2.x)
- **Core Types & Error Handling** (`types.rs`, `error.rs`)
  - Comprehensive type definitions for trading, orders, positions
  - `AppError` enum for consistent error handling
  - `Result` type for async operations

- **Configuration Management** (`config/mod.rs`)
  - `AppConfig` struct with comprehensive settings
  - `ConfigManager` for loading and managing config
  - Support for TOML files and environment variables
  - Validation and default values

- **Database Layer** (`persistence/`)
  - Repository pattern for data access
  - SQLite with SQLx for async operations
  - Support for trades, orders, balances
  - Backup and migration support

- **Exchange Integration** (`exchange/`)
  - `Exchange` trait for multiple exchange support
  - Binance exchange implementation with REST API
  - Order management and K-line data fetching
  - Market data retrieval

- **Strategy System** (`strategy/`)
  - `Strategy` trait for custom trading strategies
  - Technical indicators: SMA, EMA, RSI, MACD, Bollinger Bands
  - Strategy registry for managing strategies
  - Signal generation based on indicator analysis

- **Risk Management** (`risk/`)
  - `RiskManager` for centralized risk control
  - Cooldown protection to prevent rapid trades
  - Low profit protection to avoid small gains
  - Max drawdown protection
  - Stoploss guard for position protection

- **Risk Management Commands** (`risk_commands.rs`)
  - Tauri commands for risk protection management
  - Commands for adding/removing protections
  - `check_global_stop` and `check_pair_stop` commands

- **Backtesting Engine** (`backtest/mod.rs`)
  - `BacktestEngine` for historical strategy testing
  - Backtests with historical data
  - Profit/loss and performance metrics
  - Backtest reports with detailed statistics

- **Hyperopt/Optimization** (`optimize/`)
  - `Hyperopt` for automatic parameter tuning
  - Multiple loss functions: short, long, drawdown, sharpe
  - `Optimizer` for finding optimal parameters
  - Custom optimization spaces and constraints

- **Trading Bot Core** (`bot/mod.rs`)
  - `Bot` struct for trading automation
  - Main trading loop with async execution
  - Risk management integration
  - Start/stop bot control
  - Coordination with exchange and strategy modules

- **Tauri API Commands** (`commands.rs`)
  - `AppState` for shared application state
  - Bot control commands: start, stop, status
  - Trade management: get_open_trades, get_all_trades
  - Backtest and dashboard statistics commands
  - Config management: get, update

#### Frontend (React 19 + TypeScript)
- **TypeScript Types** (`types/index.ts`)
  - Comprehensive types for trades, config, stats

- **State Management** (`stores/appStore.ts`)
  - Zustand store for application state
  - Bot control, trade queries, dashboard stats, config management

- **API Service** (`services/api.ts`)
  - Backend communication layer

- **UI Components** (`ui/`, `components/`)
  - Base components: Button, Card
  - SidebarItem, StatCard, TradeRow, AIModal

- **Pages** (`pages/`)
  - Dashboard, Trade, Backtest, Hyperopt, Settings, Logs
  - Risk management protections page

- **Internationalization** (`i18n/`, `contexts/`, `hooks/`)
  - i18n configuration
  - LanguageContext for locale management
  - useTranslation hook

- **Main App** (`App.tsx`, `main.tsx`)
  - Trading dashboard UI with sidebar navigation
  - Bot start/stop controls with status indicators
  - React Router for multi-page navigation
  - Global styles with TailwindCSS

### Changed
- Replaced default Tauri template with custom trading dashboard
- Updated entry points (`lib.rs`, `main.rs`) to export all modules
- Added tracing for async logging
- Setup Tauri app with all commands and state management

### Tags

| Tag | Date | Description |
|-----|------|-------------|
| [v1.0.0] | 2026-01-15 | Initial release with complete trading bot implementation |

---

**Full Changelog**: [Compare v0.0.0...v1.0.0](https://github.com/freqtrade/freqtrade-rs/compare/v0.0.0...v1.0.0)

---

*Generated on 2026-01-15*

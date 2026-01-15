# CLAUDE.md

> This file provides guidance to Claude Code (or other AI assistants) when working with this repository.

## Project Overview

freqtrade-rs is a modern cryptocurrency trading bot built with Rust and Tauri. It's a complete rewrite of [freqtrade](https://github.com/freqtrade/freqtrade) (Python version), focusing on high performance, type safety, and a modern desktop application experience.

## Tech Stack

### Backend (Rust + Tauri 2.x)
- **Language**: Rust 1.70+
- **Async Runtime**: Tokio
- **Desktop Framework**: Tauri 2.x
- **Database**: SQLite with SQLx
- **Key Modules**:
  - `bot/`: Trading bot core logic
  - `exchange/`: Exchange integrations (Binance)
  - `strategy/`: Strategy system
  - `backtest/`: Backtesting engine
  - `risk/`: Risk management (100% complete)
  - `optimize/`: Parameter optimization/hyperopt
  - `persistence/`: Data persistence (Repository pattern)

### Frontend (React 19 + TypeScript)
- **Framework**: React 19
- **Language**: TypeScript 5.8
- **Build Tool**: Vite
- **Routing**: React Router v7
- **State Management**: Zustand
- **Styling**: TailwindCSS
- **Charts**: Recharts
- **Icons**: Lucide React

## Key Architecture Decisions

### 1. Async/Await Throughout
The entire backend uses async/await with Tokio runtime. All I/O operations (database, network) are non-blocking.

### 2. Repository Pattern for Database
All database operations go through `persistence::Repository`. This provides:
- Centralized data access
- Easy to mock for testing
- Consistent error handling

### 3. Arc + Mutex/RwLock for Concurrency
Shared state uses `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for thread-safe access.

### 4. Tauri Commands as API Layer
Frontend communicates with backend via Tauri commands (defined in `commands.rs` and `risk_commands.rs`).

### 5. Event-Driven Trading Loop
The trading bot runs in a continuous loop:
```
1. Check global stop (risk management)
2. Fetch K-line data
3. Generate signals (strategy)
4. Check pair stop (risk management)
5. Execute trades
```

## Project Structure

```
freqtrade-rs/
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── bot/            # Trading bot
│   │   ├── exchange/       # Exchange implementations
│   │   ├── strategy/       # Strategy system
│   │   ├── backtest/       # Backtesting engine
│   │   ├── optimize/       # Hyperopt
│   │   ├── risk/           # Risk management ✅ Complete
│   │   ├── data/           # Data management
│   │   ├── persistence/    # Database layer
│   │   ├── config/         # Configuration
│   │   ├── commands.rs     # Tauri commands
│   │   ├── main.rs         # Entry point
│   │   └── types.rs        # Core types
│   └── Cargo.toml
│
├── src/                     # React frontend
│   ├── pages/              # Route pages
│   ├── components/         # Reusable components
│   ├── services/           # API calls
│   ├── stores/             # Zustand stores
│   ├── types/              # TypeScript types
│   └── ui/                 # Base UI components
│
├── config/                 # Configuration files
├── migrations/             # Database migrations
├── user_data/              # User data directory
├── docs/                   # Documentation
└── package.json
```

## Important File Patterns

### Rust Module Pattern
Each major module follows this pattern:
```
module/
├── mod.rs          # Module entry, exports
├── module.rs       # Main implementation
├── submodule1.rs   # Sub-components
└── submodule2.rs
```

### Tauri Command Pattern
```rust
#[tauri::command]
pub async fn command_name(state: State<'_, AppState>) -> Result<ReturnType> {
    // Implementation
}
```

### Error Handling
Use `crate::error::AppError` for consistent error handling:
```rust
pub enum AppError {
    Config(String),
    Database(String),
    Exchange(String),
    Bot(String),
    Serialization(serde_json::Error),
    Parse(String),
}
```

## Development Commands

### Backend
```bash
cd src-tauri
cargo check              # Check compilation
cargo build              # Build release
cargo run                # Run dev
cargo test               # Run tests
cargo clippy             # Linting
cargo fmt                # Formatting
```

### Frontend
```bash
cd src
pnpm install             # Install dependencies
pnpm run dev             # Dev server
pnpm run build           # Production build
pnpm run lint            # Linting
```

### Full Stack
```bash
pnpm run tauri:dev       # Run both frontend + backend
pnpm run tauri:build     # Build production bundle
```

## Current Development Status

### Completed ✅
- Risk management module (100%)
- Database layer (90%)
- Binance exchange (95%)
- Loss functions (100%)
- Basic trading bot

### In Progress 🟡
- Hyperopt/optimization module (~60%)
- Strategy system (~55%)
- Order management (~65%)
- WebUI dashboard (~25%)

### Not Started 🔴
- Data downloader (0%)
- Data converter (0%)
- Telegram bot (0%)
- FreqAI/ML (0%)
- Multiple exchanges (0%)

## Key Configuration Files

| File | Purpose |
|------|---------|
| `config/default.toml` | Bot configuration |
| `.env.example` | Environment variables template |
| `src-tauri/tauri.conf.json` | Tauri configuration |
| `tailwind.config.js` | TailwindCSS configuration |
| `vite.config.ts` | Vite configuration |

## Common Patterns & Conventions

### 1. Async/Await Everywhere
Never block on async operations in the backend.

### 2. Decimal for Money
Use `rust_decimal::Decimal` for all financial calculations to avoid floating-point errors.

### 3. DateTime with UTC
All timestamps use `chrono::DateTime<Utc>`.

### 4. Error Propagation
Use `?` operator for error propagation. Return `Result<T, AppError>`.

### 5. Arc for Shared State
Wrap pools and managers in `Arc` for shared access across threads.

## Testing Strategy

### Unit Tests
Located alongside modules: `module_name.rs` → `module_name_test.rs`

### Integration Tests
In `src-tauri/tests/` directory.

### Frontend Tests
Use Vitest in `src/` directory.

## Documentation Structure

| Document | Description |
|----------|-------------|
| README.md | Project overview |
| MIGRATION_PLAN.md | Feature migration roadmap |
| docs/api/README.md | API reference |
| docs/development/ARCHITECTURE.md | Architecture details |

## Getting Help

- **freqtrade Python docs**: https://www.freqtrade.io/en/stable/
- **Tauri docs**: https://tauri.app/
- **Rust docs**: https://doc.rust-lang.org/
- **Tokio docs**: https://tokio.rs/

---

*Last updated: 2026-01-14*

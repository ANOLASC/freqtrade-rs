# Development Guide

> Comprehensive guide for setting up and working with the freqtrade-rs development environment.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Initial Setup](#initial-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Running the Application](#running-the-application)
- [Testing](#testing)
- [Building for Production](#building-for-production)
- [Debugging](#debugging)
- [Common Issues](#common-issues)
- [Tools and Utilities](#tools-and-utilities)

---

## Prerequisites

### Required Tools

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.70+ | Backend language |
| Node.js | 18+ | Frontend runtime |
| pnpm | 8+ | Frontend package manager |
| Git | 2.0+ | Version control |

### Optional Tools

| Tool | Purpose | Installation |
|------|---------|--------------|
| Tauri CLI | Desktop app development | `cargo install tauri-cli` |
| sqlx-cli | Database migrations | `cargo install sqlx-cli` |
| just | Task runner | `cargo install just` |

---

## Initial Setup

### 1. Clone the Repository

```bash
# Clone with HTTPS
git clone https://github.com/code-yeongyu/freqtrade-rs.git
cd freqtrade-rs

# Or with SSH
git clone git@github.com:code-yeongyu/freqtrade-rs.git
cd freqtrade-rs
```

### 2. Set Up Rust Environment

```bash
# Verify Rust installation
rustc --version
cargo --version

# If using Rust via rustup (recommended)
rustup default stable
rustup update stable
```

### 3. Set Up Node Environment

```bash
# Verify Node.js installation
node --version
npm --version

# Install pnpm globally
npm install -g pnpm

# Verify pnpm
pnpm --version
```

### 4. Install Dependencies

#### Rust Dependencies
```bash
cd src-tauri

# Fetch and build all Rust dependencies
cargo fetch
cargo build

# This may take a few minutes on first run
```

#### Frontend Dependencies
```bash
cd ../src

# Install all frontend dependencies
pnpm install

# Verify installation
pnpm run --version
```

### 5. Configure Environment

```bash
# Copy example environment file
cp .env.example .env

# Edit with your settings (if needed for development)
# Most settings can be left as defaults for local development
```

---

## Project Structure

### Directory Overview

```
freqtrade-rs/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── bot/                 # Trading bot core
│   │   ├── exchange/            # Exchange implementations
│   │   │   └── binance.rs       # Binance integration
│   │   ├── strategy/            # Strategy system
│   │   ├── backtest/            # Backtesting engine
│   │   ├── optimize/            # Hyperopt/optimization
│   │   ├── risk/                # Risk management
│   │   ├── data/                # Data management
│   │   ├── persistence/         # Database layer
│   │   │   └── repository.rs    # Repository pattern
│   │   ├── config/              # Configuration
│   │   ├── commands.rs          # Tauri commands
│   │   ├── main.rs              # Entry point
│   │   └── types.rs             # Core types
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                          # React frontend
│   ├── pages/                    # Route pages
│   │   ├── dashboard/
│   │   ├── trade/
│   │   └── ...
│   ├── components/               # Reusable components
│   ├── services/                 # API services
│   ├── stores/                   # Zustand stores
│   ├── types/                    # TypeScript types
│   └── ui/                       # Base UI components
│
├── config/                       # Configuration files
│   └── default.toml             # Default config
│
├── migrations/                   # Database migrations
│   └── 001_initial.sql
│
├── docs/                         # Documentation
├── user_data/                    # User data directory
└── package.json
```

### Key Files

| File | Purpose |
|------|---------|
| `src-tauri/src/main.rs` | Application entry point |
| `src-tauri/src/commands.rs` | Tauri API commands |
| `src-tauri/src/types.rs` | Core data types |
| `src/App.tsx` | Frontend entry point |
| `src/services/api.ts` | Frontend API client |
| `config/default.toml` | Bot configuration |

---

## Development Workflow

### 1. Start Development Server

#### Option A: Full Stack (Recommended)

```bash
# Terminal 1: Start Rust backend with hot reload
cd src-tauri
cargo run

# Terminal 2: Start frontend dev server
cd src
pnpm run dev
```

This will:
- Start the Tauri application in development mode
- Start the Vite development server
- Enable hot module replacement for frontend
- Enable automatic rebuild for Rust on change

#### Option B: Frontend Only (UI Development)

```bash
cd src
pnpm run dev
```

Access at `http://localhost:5173` (or shown in terminal).

### 2. Make Changes

#### Rust Changes
```bash
# Auto-rebuild on file change
cd src-tauri
cargo watch -x run
```

#### Frontend Changes
Changes are automatically reflected via Vite HMR.

### 3. Code Quality Checks

```bash
# Rust
cd src-tauri

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Frontend
cd ../src

# Type check
pnpm run type-check

# Lint
pnpm run lint

# Format
pnpm run format
```

### 4. Database Operations

```bash
# Create database
cd src-tauri
cargo run  # Creates SQLite database at configured path

# View database (using SQLite browser or CLI)
sqlite3 user_data/freqtrade.db ".tables"

# Run migrations manually
sqlx database create
sqlx migrate run
```

---

## Running the Application

### Development Mode

```bash
# Full development mode (both backend and frontend)
cd src-tauri
cargo run
```

This will:
1. Load configuration from `config/default.toml`
2. Initialize SQLite database
3. Start Tauri window with hot-reloaded frontend
4. Enable debug features and logging

### Production Mode (Testing)

```bash
# Build production bundle
cd src-tauri
cargo build --release

# Run the built binary
./target/release/freqtrade-rs
```

### Dry Run Mode

The bot runs in dry-run mode by default, which:
- Simulates trading without real money
- Uses simulated exchange data
- Logs all actions without executing real trades

To enable live trading, update `config/default.toml`:
```toml
[bot]
dry_run = false
```

---

## Testing

### Unit Tests

#### Rust Unit Tests
```bash
cd src-tauri

# Run all tests
cargo test

# Run specific module tests
cargo test risk::
cargo test optimize::

# Run with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

#### Frontend Unit Tests
```bash
cd src

# Run all tests
pnpm run test

# Watch mode
pnpm run test:watch

# Coverage report
pnpm run test:coverage
```

### Integration Tests

```bash
# Run integration tests
cd src-tauri
cargo test --test integration

# Run specific integration test
cargo test --test integration::test_name
```

### End-to-End Tests

```bash
# Run E2E tests (if configured)
cd src
pnpm run test:e2e
```

---

## Building for Production

### Build Steps

```bash
cd src-tauri

# Clean previous builds
cargo clean

# Build release
cargo build --release

# The built binary will be at:
# ./target/release/freqtrade-rs.exe (Windows)
# ./target/release/freqtrade-rs (Linux/macOS)

# Build Tauri bundle
cargo tauri build
```

### Build Output

The Tauri build process creates:
- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image
- **Linux**: `.deb` package

Output location: `src-tauri/target/release/bundle/`

### Build Verification

```bash
# Verify the binary works
./target/release/freqtrade-rs --version

# Check binary size
ls -lh target/release/freqtrade-rs
```

---

## Debugging

### Rust Debugging

#### Using `println!` (Simple)
```rust
println!("Debug info: {:?}", value);
tracing::debug!("Debug info: {:?}", value);
```

#### Using `dbg!` (Quick)
```rust
let result = dbg!(some_function());
```

#### Using Log Levels
```rust
tracing::error!("This is an error");
tracing::warn!("This is a warning");
tracing::info!("This is info");
tracing::debug!("This is debug");
tracing::trace!("This is trace");
```

### Frontend Debugging

#### Browser DevTools
Open browser DevTools (F12) for:
- React component inspection
- Network request monitoring
- Console logging

#### React DevTools
Install React DevTools browser extension for component tree inspection.

### Debug Logs

Logs are written to:
- **Console**: Standard output
- **File**: Check `user_data/logs/` directory

### Common Debugging Commands

```bash
# Enable verbose logging
RUST_LOG=debug cargo run

# Enable trace logging
RUST_LOG=trace cargo run

# Frontend verbose mode
DEBUG=1 pnpm run dev
```

---

## Common Issues

### Issue: Cargo Build Fails

**Error**: `could not compile ...`

**Solution**:
```bash
# Clear build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

### Issue: Frontend Build Fails

**Error**: `Module not found` or TypeScript errors

**Solution**:
```bash
# Clear node_modules
rm -rf node_modules

# Reinstall
pnpm install

# Type check
pnpm run type-check
```

### Issue: Database Locked

**Error**: `database is locked`

**Solution**:
- Ensure only one instance is running
- Check for hanging processes
- Delete lock file if exists

### Issue: Tauri Window Doesn't Open

**Solution**:
```bash
# Check for port conflicts
lsof -i :5173

# Run in debug mode
cargo run -- --debug
```

### Issue: API Keys Not Working

**Check**:
1. Keys are in `.env` file
2. Keys have correct permissions
3. Using testnet for testing

```bash
# Verify env vars are loaded
cd src-tauri
cargo run -- --print-env
```

---

## Tools and Utilities

### Recommended VS Code Extensions

| Extension | Purpose |
|-----------|---------|
| rust-analyzer | Rust development |
| Tauri | Tauri app development |
| ESLint | JavaScript/TypeScript linting |
| Prettier | Code formatting |
| GitLens | Git history |

### Git Aliases

Add to `.git/config` or use global git config:
```bash
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.df diff
```

### Shell Aliases (Add to `~/.bashrc` or `~/.zshrc`)

```bash
alias fr='cd /path/to/freqtrade-rs'
alias fr-dev='cd src-tauri && cargo run'
alias fr-front='cd src && pnpm run dev'
alias fr-test='cd src-tauri && cargo test'
alias fr-lint='cd src-tauri && cargo clippy'
alias fr-fmt='cd src-tauri && cargo fmt'
```

---

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [Tokio Async Runtime](https://tokio.rs/)
- [SQLx Database](https://github.com/launchbadge/sqlx)

---

*Last updated: 2026-01-14*

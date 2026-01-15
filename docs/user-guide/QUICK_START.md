# Quick Start Guide

> Get up and running with freqtrade-rs in 5 minutes.

## Prerequisites

- **Windows 10+** / **macOS 10.15+** / **Linux**
- **8GB RAM** minimum (16GB recommended)
- **10GB free disk space**

### Required Software

| Software | Version | How to Install |
|----------|---------|----------------|
| Rust | 1.70+ | [rustup.rs](https://rustup.rs/) |
| Node.js | 18+ | [nodejs.org](https://nodejs.org/) |
| pnpm | 8+ | `npm install -g pnpm` |

---

## Step 1: Install Dependencies

### Install Rust
```bash
# Download and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your terminal or run:
source ~/.cargo/env

# Verify installation
rustc --version  # Should show 1.70 or later
```

### Install Node.js and pnpm
```bash
# Using nvm (recommended)
nvm install 18
nvm use 18

# Install pnpm
npm install -g pnpm

# Verify
pnpm --version
```

---

## Step 2: Clone and Setup

```bash
# Clone the repository
git clone https://github.com/code-yeongyu/freqtrade-rs.git
cd freqtrade-rs

# Install Rust dependencies
cd src-tauri
cargo fetch
cargo build

# Install frontend dependencies
cd ../src
pnpm install
```

---

## Step 3: Configure Your Environment

```bash
# Copy example environment file
cp .env.example .env

# Edit if needed (not required for basic testing)
# nano .env
```

**Note**: For dry-run mode, no API keys are needed.

---

## Step 4: Run the Application

### Option A: Full Development Mode (Recommended)

**Terminal 1** - Start the backend:
```bash
cd src-tauri
cargo run
```

**Terminal 2** - Start the frontend:
```bash
cd src
pnpm run dev
```

Open `http://localhost:5173` in your browser.

### Option B: Single Command

```bash
# Run both backend and frontend
cd src-tauri
cargo run
```

The frontend will automatically open in the Tauri window.

---

## Step 5: Verify Installation

After the application starts, you should see:

1. **Tauri Window** opens with the dashboard
2. **Bot Status** shows "Stopped" (default)
3. **No open trades** (fresh installation)

### Test the Application

1. Click **"Start Bot"** button
2. Watch the status change to "Running"
3. Check the **Dashboard** for real-time stats
4. Click **"Stop Bot"** to stop

---

## Understanding the Dashboard

### Main Sections

| Section | Description |
|---------|-------------|
| **Status** | Bot running/stopped state |
| **Stats** | Total profit, win rate, open trades |
| **Equity Curve** | Profit/loss over time |
| **Active Trades** | Currently open positions |

### Navigation

- **Dashboard** - Main overview
- **Backtest** - Strategy testing
- **Hyperopt** - Parameter optimization
- **Settings** - Configuration
- **Logs** - System logs

---

## Running Your First Backtest

1. Go to **Backtest** tab
2. Select a strategy (e.g., "SimpleStrategy")
3. Choose timeframe (e.g., "1h")
4. Set date range (e.g., last 30 days)
5. Click **"Run Backtest"**
6. View results in the dashboard

---

## Troubleshooting

### Issue: "Cargo not found"

**Solution**:
```bash
# Add Rust to your PATH
source ~/.cargo/env

# Or restart your terminal
```

### Issue: "Node modules not found"

**Solution**:
```bash
cd src
pnpm install
```

### Issue: "Port 5173 already in use"

**Solution**:
```bash
# Find and kill the process
lsof -i :5173
kill -9 <PID>

# Or use a different port
pnpm run dev -- --port 3000
```

### Issue: "Database locked"

**Solution**:
- Close all instances of the application
- Delete `user_data/freqtrade.db` (fresh start)
- Restart the application

---

## Next Steps

### Learn the Basics

- [Configuration Guide](CONFIGURATION.md) - Configure for live trading
- [Strategy Guide](STRATEGIES.md) - Create custom strategies
- [API Reference](../api/README.md) - Use the API

### Explore Features

- [Backtesting](BACKTESTING.md) - Test strategies historically
- [Risk Management](RISK_MANAGEMENT.md) - Set up protections
- [Hyperopt](HYPEROPT.md) - Optimize parameters

---

## Getting Help

| Resource | Link |
|----------|------|
| GitHub Issues | [github.com/code-yeongyu/freqtrade-rs/issues](https://github.com/code-yeongyu/freqtrade-rs/issues) |
| Discord | [discord.gg/freqtrade](https://discord.gg/p7nuUxk) |
| Wiki | [github.com/code-yeongyu/freqtrade-rs/wiki](https://github.com/code-yeongyu/freqtrade-rs/wiki) |

---

## What's Next?

Congratulations! You've successfully set up freqtrade-rs.

**Recommended reading**:
1. [Configuration Guide](CONFIGURATION.md) - Set up for live trading
2. [Strategy Guide](STRATEGIES.md) - Learn to create strategies
3. [Risk Management](RISK_MANAGEMENT.md) - Protect your capital

Happy trading! 🚀

---

*Last updated: 2026-01-14*

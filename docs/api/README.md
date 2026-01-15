# API Reference

> Complete reference for freqtrade-rs Tauri API commands.

## Overview

freqtrade-rs uses Tauri commands to provide a JSON-RPC style API between the frontend (React) and backend (Rust). All commands are asynchronous and return `Result<T, AppError>`.

## Base Configuration

### AppState

All commands have access to the `AppState`:

```typescript
interface AppState {
  config: Arc<RwLock<AppConfig>>;
  repository: Arc<Repository>;
  bot: Arc<Mutex<Option<TradingBot>>>;
  risk_manager: Arc<RwLock<Option<Arc<RiskManager>>>>;
}
```

---

## Bot Control Commands

### get_bot_status

Retrieves the current status of the trading bot.

**Parameters**: None

**Returns**: `BotStatus`

```typescript
enum BotStatus {
  Stopped = "Stopped",
  Running = "Running",
  Paused = "Paused",
  Error = "Error",
}
```

**Example**:
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const status = await invoke<BotStatus>('get_bot_status');
console.log(status); // "Running"
```

**Errors**:
- `AppError::Bot` - If bot state is invalid

---

### start_bot

Starts the trading bot with configured exchange and strategy.

**Parameters**: None

**Returns**: `Result<string>`

**Side Effects**:
- Initializes Binance exchange connection
- Creates RiskManager with default protections
- Starts trading loop in background task

**Example**:
```typescript
const result = await invoke<string>('start_bot');
console.log(result); // "Bot started with risk management"
```

**Errors**:
- `AppError::Bot` - "Bot is already running"
- `AppError::Config` - Invalid configuration
- `AppError::Exchange` - Exchange connection failed

---

### stop_bot

Stops the trading bot gracefully.

**Parameters**: None

**Returns**: `Result<string>`

**Side Effects**:
- Stops trading loop
- Closes open positions (if configured)
- Saves state to database

**Example**:
```typescript
const result = await invoke<string>('stop_bot');
console.log(result); // "Bot stopped"
```

---

## Trade Management Commands

### get_open_trades

Retrieves all currently open trades.

**Parameters**: None

**Returns**: `Result<Vec<Trade>>`

**Example**:
```typescript
interface Trade {
  id: string;           // UUID
  pair: string;         // e.g., "BTC/USDT"
  is_open: boolean;
  exchange: string;
  open_rate: number;
  open_date: string;    // ISO datetime
  close_rate?: number;
  close_date?: string;
  amount: number;
  stake_amount: number;
  strategy: string;
  timeframe: Timeframe;
  stop_loss?: number;
  take_profit?: number;
  exit_reason?: ExitType;
  profit_abs?: number;
  profit_ratio?: number;
}

const trades = await invoke<Trade[]>('get_open_trades');
```

---

### get_all_trades

Retrieves all trades (both open and closed).

**Parameters**: None

**Returns**: `Result<Vec<Trade>>`

**Example**:
```typescript
const allTrades = await invoke<Trade[]>('get_all_trades');
```

---

## Backtesting Commands

### run_backtest

Runs a backtest with the specified configuration.

**Parameters**:
```typescript
interface BacktestConfig {
  strategy: string;
  timeframe: Timeframe;
  start_date: string;  // ISO datetime
  end_date: string;    // ISO datetime
  pairs: string[];
}
```

**Returns**: `Result<BacktestResult>`

**Example**:
```typescript
const config: BacktestConfig = {
  strategy: 'SimpleStrategy',
  timeframe: '1h',
  start_date: '2024-01-01T00:00:00Z',
  end_date: '2024-12-31T23:59:59Z',
  pairs: ['BTC/USDT', 'ETH/USDT'],
};

const result = await invoke<BacktestResult>('run_backtest', { config });
```

**BacktestResult**:
```typescript
interface BacktestResult {
  strategy: string;
  pair: string;
  timeframe: Timeframe;
  start_date: string;
  end_date: string;
  total_trades: number;
  winning_trades: number;
  losing_trades: number;
  win_rate: number;
  total_profit: number;
  max_drawdown: number;
  sharpe_ratio: number;
  profit_factor: number;
  avg_profit: number;
  avg_loss: number;
  trades: Trade[];
}
```

---

## Dashboard Commands

### get_dashboard_stats

Retrieves dashboard statistics for the UI.

**Parameters**: None

**Returns**: `Result<DashboardStats>`

**Example**:
```typescript
interface DashboardStats {
  total_profit: number;
  win_rate: number;
  open_trades: number;
  max_drawdown: number;
  total_balance: number;
}

const stats = await invoke<DashboardStats>('get_dashboard_stats');
```

---

### get_equity_curve

Retrieves equity curve data for charting.

**Parameters**: None

**Returns**: `Result<Vec<EquityPoint>>`

**Example**:
```typescript
interface EquityPoint {
  time: string;
  value: number;
}

const equityCurve = await invoke<EquityPoint[]>('get_equity_curve');
```

---

## Configuration Commands

### get_config

Retrieves the current bot configuration.

**Parameters**: None

**Returns**: `Result<AppConfig>`

**Example**:
```typescript
interface AppConfig {
  bot: BotConfig;
  exchange: ExchangeConfig;
  strategy: StrategyConfig;
  database: DatabaseConfig;
}

const config = await invoke<AppConfig>('get_config');
```

---

### update_config

Updates the bot configuration.

**Parameters**:
```typescript
interface UpdateConfigRequest {
  config: Partial<AppConfig>;
}
```

**Returns**: `Result<string>`

**Side Effects**:
- Updates configuration in memory
- May trigger strategy reload
- Does NOT persist to disk (use `save_config`)

**Example**:
```typescript
const result = await invoke<string>('update_config', {
  config: {
    bot: {
      max_open_trades: 5,
      stake_amount: 100.0,
    },
  },
});
```

---

## Risk Management Commands

### add_cooldown_protection

Adds a cooldown period protection mechanism.

**Parameters**:
```typescript
interface CooldownPeriodConfig {
  stop_duration: number;   // minutes
  lookback_period: number; // minutes
  stop_after_losses: number;
}
```

**Returns**: `Result<string>`

**Example**:
```typescript
const config = {
  stop_duration: 60,    // Stop for 60 minutes
  lookback_period: 1440, // Look back 24 hours
  stop_after_losses: 2,  // After 2 losing trades
};

const result = await invoke<string>('add_cooldown_protection', { config });
```

---

### add_low_profit_protection

Adds protection for low-profit pairs.

**Parameters**:
```typescript
interface LowProfitPairsConfig {
  stop_duration: number;    // minutes
  lookback_period: number;  // minutes
  required_profit: number;  // percentage
  required_trades: number;
}
```

**Returns**: `Result<string>`

**Example**:
```typescript
const config = {
  stop_duration: 60,
  lookback_period: 1440,
  required_profit: 0.01,   // 1% minimum
  required_trades: 5,
};

const result = await invoke<string>('add_low_profit_protection', { config });
```

---

### add_max_drawdown_protection

Adds maximum drawdown protection.

**Parameters**:
```typescript
interface MaxDrawdownProtectionConfig {
  max_allowed_drawdown: number; // percentage
  lookback_period: number;      // minutes
  stop_duration: number;        // minutes
}
```

**Returns**: `Result<string>`

**Example**:
```typescript
const config = {
  max_allowed_drawdown: 20.0,  // 20% max drawdown
  lookback_period: 1440,
  stop_duration: 60,
};

const result = await invoke<string>('add_max_drawdown_protection', { config });
```

---

### add_stoploss_guard

Adds stoploss guard protection.

**Parameters**:
```typescript
interface StoplossGuardConfig {
  lookback_period: number;   // minutes
  stop_duration: number;     // minutes
  max_stoploss_count: number;
}
```

**Returns**: `Result<string>`

**Example**:
```typescript
const config = {
  lookback_period: 60,
  stop_duration: 30,
  max_stoploss_count: 5,
};

const result = await invoke<string>('add_stoploss_guard', { config });
```

---

### list_protections

Lists all active protection mechanisms.

**Parameters**: None

**Returns**: `Result<Vec<ProtectionInfo>>`

**Example**:
```typescript
interface ProtectionInfo {
  name: string;
  type: string;
  config: object;
  is_active: boolean;
}

const protections = await invoke<ProtectionInfo[]>('list_protections');
```

---

### remove_protection

Removes a protection mechanism.

**Parameters**:
```typescript
interface RemoveProtectionRequest {
  name: string;
}
```

**Returns**: `Result<string>`

**Example**:
```typescript
const result = await invoke<string>('remove_protection', { name: 'cooldown_1' });
```

---

### check_global_stop

Checks if global stop is active.

**Parameters**: None

**Returns**: `Result<ProtectionReturn>`

**Example**:
```typescript
interface ProtectionReturn {
  lock: boolean;
  until?: string;
  reason?: string;
  lock_side: string;
}

const result = await invoke<ProtectionReturn>('check_global_stop');
```

---

### check_pair_stop

Checks if stop is active for a specific pair.

**Parameters**:
```typescript
interface CheckPairStopRequest {
  pair: string;
}
```

**Returns**: `Result<ProtectionReturn>`

**Example**:
```typescript
const result = await invoke<ProtectionReturn>('check_pair_stop', { pair: 'BTC/USDT' });
```

---

## Type Definitions

### Timeframe

```typescript
enum Timeframe {
  OneMinute = "1m",
  ThreeMinutes = "3m",
  FiveMinutes = "5m",
  FifteenMinutes = "15m",
  ThirtyMinutes = "30m",
  OneHour = "1h",
  TwoHours = "2h",
  FourHours = "4h",
  SixHours = "6h",
  EightHours = "8h",
  TwelveHours = "12h",
  OneDay = "1d",
  ThreeDays = "3d",
  OneWeek = "1w",
  OneMonth = "1M",
}
```

### ExitType

```typescript
enum ExitType {
  Signal = "signal",
  StopLoss = "stop_loss",
  TakeProfit = "take_profit",
  StopLossOnExchange = "stop_loss_on_exchange",
  ForceExit = "force_exit",
  EmergencyExit = "emergency_exit",
  Custom = "custom",
}
```

### TradeSide

```typescript
enum TradeSide {
  Buy = "Buy",
  Sell = "Sell",
}
```

### OrderType

```typescript
enum OrderType {
  Market = "market",
  Limit = "limit",
  StopLimit = "stop_limit",
  StopMarket = "stop_market",
}
```

### OrderStatus

```typescript
enum OrderStatus {
  New = "new",
  PartiallyFilled = "partially_filled",
  Filled = "filled",
  Canceled = "canceled",
  Rejected = "rejected",
  Expired = "expired",
}
```

---

## Error Handling

All commands return `Result<T, AppError>`:

```typescript
interface AppError {
  type: 'Config' | 'Database' | 'Exchange' | 'Bot' | 'Serialization' | 'Parse';
  message: string;
}
```

**Example Error Handling**:
```typescript
try {
  const result = await invoke<string>('start_bot');
  console.log('Success:', result);
} catch (error) {
  if (error && typeof error === 'object' && 'type' in error) {
    const appError = error as AppError;
    console.error(`Error [${appError.type}]: ${appError.message}`);
  } else {
    console.error('Unknown error:', error);
  }
}
```

---

## Frontend Usage Example

```typescript
// src/services/api.ts
import { invoke } from '@tauri-apps/api/tauri';

export const api = {
  // Bot control
  getBotStatus: () => invoke('get_bot_status'),
  startBot: () => invoke('start_bot'),
  stopBot: () => invoke('stop_bot'),

  // Trade management
  getOpenTrades: () => invoke('get_open_trades'),
  getAllTrades: () => invoke('get_all_trades'),

  // Backtesting
  runBacktest: (config: BacktestConfig) => invoke('run_backtest', { config }),

  // Dashboard
  getDashboardStats: () => invoke('get_dashboard_stats'),
  getEquityCurve: () => invoke('get_equity_curve'),

  // Configuration
  getConfig: () => invoke('get_config'),
  updateConfig: (config: Partial<AppConfig>) => invoke('update_config', { config }),

  // Risk management
  addCooldownProtection: (config: CooldownPeriodConfig) =>
    invoke('add_cooldown_protection', { config }),
  addLowProfitProtection: (config: LowProfitPairsConfig) =>
    invoke('add_low_profit_protection', { config }),
  addMaxDrawdownProtection: (config: MaxDrawdownProtectionConfig) =>
    invoke('add_max_drawdown_protection', { config }),
  addStoplossGuard: (config: StoplossGuardConfig) =>
    invoke('add_stoploss_guard', { config }),
  listProtections: () => invoke('list_protections'),
  removeProtection: (name: string) => invoke('remove_protection', { name }),
  checkGlobalStop: () => invoke('check_global_stop'),
  checkPairStop: (pair: string) => invoke('check_pair_stop', { pair }),
};
```

---

## Rate Limiting

The API does not implement rate limiting at the command level. However, the exchange integration implements rate limiting to comply with exchange API limits.

---

## Versioning

This API follows semantic versioning. Breaking changes will increment the major version.

**Current Version**: 0.1.0

---

*Last updated: 2026-01-14*

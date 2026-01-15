# API 参考

> freqtrade-rs Tauri API 命令的完整参考。

## 概述

freqtrade-rs 使用 Tauri 命令在前端（React）和后端（Rust）之间提供 JSON-RPC 风格的 API。所有命令都是异步的，返回 `Result<T, AppError>`。

## 基础配置

### AppState

所有命令都可以访问 `AppState`：

```typescript
interface AppState {
  config: Arc<RwLock<AppConfig>>;
  repository: Arc<Repository>;
  bot: Arc<Mutex<Option<TradingBot>>>;
  risk_manager: Arc<RwLock<Option<Arc<RiskManager>>>>;
}
```

---

## 机器人控制命令

### get_bot_status

获取交易机器人的当前状态。

**参数**：无

**返回**：`BotStatus`

```typescript
enum BotStatus {
  Stopped = "Stopped",
  Running = "Running",
  Paused = "Paused",
  Error = "Error",
}
```

**示例**：
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const status = await invoke<BotStatus>('get_bot_status');
console.log(status); // "Running"
```

**错误**：
- `AppError::Bot` - 如果机器人状态无效

---

### start_bot

启动交易机器人（已配置交易所和策略）。

**参数**：无

**返回**：`Result<string>`

**副作用**：
- 初始化 Binance 交易所连接
- 创建带有默认保护的 RiskManager
- 在后台任务中启动交易循环

**示例**：
```typescript
const result = await invoke<string>('start_bot');
console.log(result); // "Bot started with risk management"
```

**错误**：
- `AppError::Bot` - "Bot is already running"
- `AppError::Config` - 无效配置
- `AppError::Exchange` - 交易所连接失败

---

### stop_bot

优雅地停止交易机器人。

**参数**：无

**返回**：`Result<string>`

**副作用**：
- 停止交易循环
- 关闭未平仓头寸（如果配置了）
- 保存状态到数据库

**示例**：
```typescript
const result = await invoke<string>('stop_bot');
console.log(result); // "Bot stopped"
```

---

## 交易管理命令

### get_open_trades

获取所有当前未平仓的交易。

**参数**：无

**返回**：`Result<Vec<Trade>>`

**示例**：
```typescript
interface Trade {
  id: string;           // UUID
  pair: string;         // 例如 "BTC/USDT"
  is_open: boolean;
  exchange: string;
  open_rate: number;
  open_date: string;    // ISO 日期时间
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

获取所有交易（包括已平仓的）。

**参数**：无

**返回**：`Result<Vec<Trade>>`

**示例**：
```typescript
const allTrades = await invoke<Trade[]>('get_all_trades');
```

---

## 回测命令

### run_backtest

使用指定配置运行回测。

**参数**：
```typescript
interface BacktestConfig {
  strategy: string;
  timeframe: Timeframe;
  start_date: string;  // ISO 日期时间
  end_date: string;    // ISO 日期时间
  pairs: string[];
}
```

**返回**：`Result<BacktestResult>`

**示例**：
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

**BacktestResult**：
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

## 仪表板命令

### get_dashboard_stats

获取仪表板的统计数据。

**参数**：无

**返回**：`Result<DashboardStats>`

**示例**：
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

获取权益曲线数据用于图表显示。

**参数**：无

**返回**：`Result<Vec<EquityPoint>>`

**示例**：
```typescript
interface EquityPoint {
  time: string;
  value: number;
}

const equityCurve = await invoke<EquityPoint[]>('get_equity_curve');
```

---

## 配置命令

### get_config

获取当前机器人配置。

**参数**：无

**返回**：`Result<AppConfig>`

**示例**：
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

更新机器人配置。

**参数**：
```typescript
interface UpdateConfigRequest {
  config: Partial<AppConfig>;
}
```

**返回**：`Result<string>`

**副作用**：
- 更新内存中的配置
- 可能触发策略重载
- **不会**保存到磁盘（使用 `save_config`）

**示例**：
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

## 风险管理命令

### add_cooldown_protection

添加冷却期保护机制。

**参数**：
```typescript
interface CooldownPeriodConfig {
  stop_duration: number;   // 分钟
  lookback_period: number; // 分钟
  stop_after_losses: number;
}
```

**返回**：`Result<string>`

**示例**：
```typescript
const config = {
  stop_duration: 60,    // 停止 60 分钟
  lookback_period: 1440, // 回看 24 小时
  stop_after_losses: 2,  // 2 次亏损交易后
};

const result = await invoke<string>('add_cooldown_protection', { config });
```

---

### add_low_profit_protection

添加低利润交易对保护。

**参数**：
```typescript
interface LowProfitPairsConfig {
  stop_duration: number;    // 分钟
  lookback_period: number;  // 分钟
  required_profit: number;  // 百分比
  required_trades: number;
}
```

**返回**：`Result<string>`

**示例**：
```typescript
const config = {
  stop_duration: 60,
  lookback_period: 1440,
  required_profit: 0.01,   // 最低 1%
  required_trades: 5,
};

const result = await invoke<string>('add_low_profit_protection', { config });
```

---

### add_max_drawdown_protection

添加最大回撤保护。

**参数**：
```typescript
interface MaxDrawdownProtectionConfig {
  max_allowed_drawdown: number; // 百分比
  lookback_period: number;      // 分钟
  stop_duration: number;        // 分钟
}
```

**返回**：`Result<string>`

**示例**：
```typescript
const config = {
  max_allowed_drawdown: 20.0,  // 最大回撤 20%
  lookback_period: 1440,
  stop_duration: 60,
};

const result = await invoke<string>('add_max_drawdown_protection', { config });
```

---

### add_stoploss_guard

添加止损保护。

**参数**：
```typescript
interface StoplossGuardConfig {
  lookback_period: number;   // 分钟
  stop_duration: number;     // 分钟
  max_stoploss_count: number;
}
```

**返回**：`Result<string>`

**示例**：
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

列出所有活动的保护机制。

**参数**：无

**返回**：`Result<Vec<ProtectionInfo>>`

**示例**：
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

移除保护机制。

**参数**：
```typescript
interface RemoveProtectionRequest {
  name: string;
}
```

**返回**：`Result<string>`

**示例**：
```typescript
const result = await invoke<string>('remove_protection', { name: 'cooldown_1' });
```

---

### check_global_stop

检查全局停止是否激活。

**参数**：无

**返回**：`Result<ProtectionReturn>`

**示例**：
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

检查特定交易对是否停止。

**参数**：
```typescript
interface CheckPairStopRequest {
  pair: string;
}
```

**返回**：`Result<ProtectionReturn>`

**示例**：
```typescript
const result = await invoke<ProtectionReturn>('check_pair_stop', { pair: 'BTC/USDT' });
```

---

## 类型定义

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

## 错误处理

所有命令返回 `Result<T, AppError>`：

```typescript
interface AppError {
  type: 'Config' | 'Database' | 'Exchange' | 'Bot' | 'Serialization' | 'Parse';
  message: string;
}
```

**错误处理示例**：
```typescript
try {
  const result = await invoke<string>('start_bot');
  console.log('成功:', result);
} catch (error) {
  if (error && typeof error === 'object' && 'type' in error) {
    const appError = error as AppError;
    console.error(`错误 [${appError.type}]: ${appError.message}`);
  } else {
    console.error('未知错误:', error);
  }
}
```

---

## 前端使用示例

```typescript
// src/services/api.ts
import { invoke } from '@tauri-apps/api/tauri';

export const api = {
  // 机器人控制
  getBotStatus: () => invoke('get_bot_status'),
  startBot: () => invoke('start_bot'),
  stopBot: () => invoke('stop_bot'),

  // 交易管理
  getOpenTrades: () => invoke('get_open_trades'),
  getAllTrades: () => invoke('get_all_trades'),

  // 回测
  runBacktest: (config: BacktestConfig) => invoke('run_backtest', { config }),

  // 仪表板
  getDashboardStats: () => invoke('get_dashboard_stats'),
  getEquityCurve: () => invoke('get_equity_curve'),

  // 配置
  getConfig: () => invoke('get_config'),
  updateConfig: (config: Partial<AppConfig>) => invoke('update_config', { config }),

  // 风险管理
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

## 速率限制

API 不会在命令级别实现速率限制。但是，交易所集成会实现速率限制以遵守交易所 API 限制。

---

## 版本控制

此 API 遵循语义化版本控制。重大更改将递增主版本号。

**当前版本**: 0.1.0

---

*最后更新：2026-01-14*

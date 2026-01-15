# freqtrade-rs 问题追踪文档

> **创建日期**: 2026-01-14  
> **最后更新**: 2026-01-14  
> **状态**: 进行中  
> **严重性**: 高 - 这些问题阻碍了核心交易功能

---

## 📋 问题总览

| 模块 | 问题数量 | 严重性 | 状态 |
|------|----------|--------|------|
| Bot 模块 | 3 | 🔴 高 | 待修复 |
| 交易所集成 | 7 | 🔴 高 | 待修复 |
| 回测系统 | 3 | 🔴 高 | 待修复 |
| 策略系统 | 2 | 🔴 高 | 待修复 |

---

## 🔴 Bot 模块问题

### BOT-001: 硬编码交易对

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/bot/mod.rs:36`

**问题描述**:
```rust
let default_pair = "BTCUSDT";
```

交易对被硬编码为 "BTCUSDT"，不支持配置多个交易对。

**影响**:
- 用户无法配置想要交易的交易对
- 只能交易单一交易对
- 违反了模块化设计原则

**修复建议**:
```rust
// 从配置中读取交易对列表
let pairs: Vec<String> = self.config.trading_pairs.clone();
```

**配置示例**:
```toml
[bot]
trading_pairs = ["BTC/USDT", "ETH/USDT", "SOL/USDT"]
```

---

### BOT-002: 买卖逻辑仅打印日志

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/bot/mod.rs:112-139`

**问题描述**:
```rust
// 执行卖出（dry_run 模式下只记录）
if self.config.dry_run {
    eprintln!("[DRY RUN] Would sell {}", pair);
} else {
    // TODO: 实现实际的卖出逻辑
    eprintln!("Sell signal for {}", pair);
}
```

买卖逻辑只打印日志，没有任何实际执行。

**影响**:
- 机器人无法进行实际交易
- 即使关闭 `dry_run` 也不会执行真实交易
- 浪费了获取的信号

**修复建议**:
```rust
if self.config.dry_run {
    eprintln!("[DRY RUN] Would sell {}", pair);
    // 记录模拟交易到数据库
    self.repository.create_trade(&trade).await?;
} else {
    // 实际执行卖出订单
    let order = self.exchange.create_order(&order_request).await?;
    eprintln!("Order executed: {:?}", order);
}
```

---

### BOT-003: 缺少多交易对支持

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/bot/mod.rs:94-144`

**问题描述**:
`process_cycle` 方法只处理单个交易对，没有循环处理多个交易对。

**当前流程**:
```
循环 → 检查单个交易对 → 处理买卖信号
```

**期望流程**:
```
循环 → 获取交易对列表 → 遍历每个交易对 → 分别处理买卖信号
```

**修复建议**:
```rust
async fn process_all_pairs(&self) -> Result<()> {
    let pairs = self.config.trading_pairs.clone();
    
    for pair in pairs {
        self.process_cycle(&pair, &self.config.timeframe).await?;
    }
    
    Ok(())
}
```

---

## 🔴 交易所集成问题

### EXCH-001: fetch_balance 未实现

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/binance.rs:83-85`

**代码**:
```rust
async fn fetch_balance(&self) -> Result<Balance> {
    Err(AppError::NotImplemented("fetch_balance not implemented yet".to_string()))
}
```

**影响**: 无法获取账户余额，无法计算可用资金。

---

### EXCH-002: fetch_positions 未实现

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/binance.rs:87-89`

**代码**:
```rust
async fn fetch_positions(&self) -> Result<Vec<Position>> {
    Err(AppError::NotImplemented("fetch_positions not implemented yet".to_string()))
}
```

**影响**: 无法获取当前持仓信息。

---

### EXCH-003: create_order 未实现

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/binance.rs:91-93`

**代码**:
```rust
async fn create_order(&self, _order: OrderRequest) -> Result<Order> {
    Err(AppError::NotImplemented("create_order not implemented yet".to_string()))
}
```

**影响**: 无法创建订单，无法执行实际交易。

---

### EXCH-004: cancel_order 未实现

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/binance.rs:95-97`

**代码**:
```rust
async fn cancel_order(&self, _order_id: &str) -> Result<()> {
    Err(AppError::NotImplemented("cancel_order not implemented yet".to_string()))
}
```

**影响**: 无法取消订单。

---

### EXCH-005: fetch_order 未实现

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/binance.rs:99-101`

**代码**:
```rust
async fn fetch_order(&self, _order_id: &str) -> Result<Order> {
    Err(AppError::NotImplemented("fetch_order not implemented yet".to_string()))
}
```

**影响**: 无法查询订单状态。

---

### EXCH-006: fetch_orders 未实现

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/binance.rs:103-105`

**代码**:
```rust
async fn fetch_orders(&self, _symbol: &str) -> Result<Vec<Order>> {
    Err(AppError::NotImplemented("fetch_orders not implemented yet".to_string()))
}
```

**影响**: 无法获取订单列表。

---

### EXCH-007: 缺少 WebSocket 实时数据支持

**严重性**: 🔴 高  
**状态**: ❌ 未实现  
**文件**: `src-tauri/src/exchange/` 目录

**问题描述**:
当前没有任何 WebSocket 实现，无法获取实时价格更新和订单状态推送。

**影响**:
- 只能轮询 REST API，延迟高
- 无法实现实时交易决策
- 无法及时响应市场变化

**实现建议**:
```rust
#[async_trait]
impl WebSocketHandler for BinanceExchange {
    async fn connect(&self, stream_type: StreamType) -> Result<WebSocketStream>;
    async fn subscribe(&self, channels: &[String]) -> Result<()>;
    async fn on_message(&self, callback: impl Fn(Message)) -> Result<()>;
}
```

---

## 🔴 回测系统问题

### BACK-001: 利润计算公式错误

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/backtest/mod.rs:77-78`

**当前代码**:
```rust
let profit = (trade.amount * candle.close).to_f64().unwrap_or(0.0);
balance = profit * (1.0 - self.config.commission);
```

**问题分析**:
1. `profit` 计算的是当前价值，不是利润
2. `balance` 被重置为 0.0 后又设置为当前价值，逻辑混乱
3. 没有计算实际盈亏

**正确公式**:
```rust
// 计算入场成本
let cost = trade.amount * trade.open_rate;
let revenue = trade.amount * candle.close;
let profit = revenue - cost;
let commission = revenue * self.config.commission;
balance = previous_balance + profit - commission;
```

---

### BACK-002: profit_abs 和 profit_ratio 未设置

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/backtest/mod.rs:70-81`

**问题描述**:
平仓时没有设置 `profit_abs` 和 `profit_ratio`，导致胜率计算不正确。

**当前代码**:
```rust
trade.is_open = false;
trade.close_rate = Some(candle.close);
trade.close_date = Some(candle.timestamp);
trade.exit_reason = Some(ExitType::Signal);
// profit_abs 和 profit_ratio 从未设置！
```

**修复建议**:
```rust
let profit_abs = revenue - cost;
let profit_ratio = profit_abs / cost;

trade.is_open = false;
trade.close_rate = Some(candle.close);
trade.close_date = Some(candle.timestamp);
trade.exit_reason = Some(ExitType::Signal);
trade.profit_abs = Some(profit_abs);
trade.profit_ratio = Some(profit_ratio);
```

---

### BACK-003: 胜率计算不准确

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/backtest/mod.rs:84-85`

**当前代码**:
```rust
let winning_trades = trades.iter()
    .filter(|t| !t.is_open && t.profit_abs.map(|p| p > rust_decimal::Decimal::ZERO).unwrap_or(false))
    .count();
let losing_trades = trades.iter()
    .filter(|t| !t.is_open && t.profit_abs.map(|p| p <= rust_decimal::Decimal::ZERO).unwrap_or(true))
    .count();
```

**问题**:
由于 BACK-002 中 `profit_abs` 从未设置，`unwrap_or(false)` 和 `unwrap_or(true)` 导致所有交易都被分类为亏损交易。

**影响**:
- 胜率始终为 0%
- 回测结果完全不可信

---

## 🔴 策略系统问题

### STRAT-001: SimpleStrategy 所有方法返回空值

**严重性**: 🔴 高  
**状态**: ❌ 待修复  
**文件**: `src-tauri/src/commands.rs:160-170`

**当前代码**:
```rust
async fn populate_indicators(&mut self, _data: &mut Vec<OHLCV>) -> Result<()> {
    Ok(())  // 没有计算任何指标
}

async fn populate_buy_trend(&self, _data: &[OHLCV]) -> Result<Vec<Signal>> {
    Ok(vec![])  // 永远不产生买入信号
}

async fn populate_sell_trend(&self, _data: &[OHLCV]) -> Result<Vec<Signal>> {
    Ok(vec![])  // 永远不产生卖出信号
}
```

**影响**:
- SimpleStrategy 无法生成任何交易信号
- 使用此策略的回测永远不会有任何交易
- 用户无法看到策略的实际效果

---

### STRAT-002: 技术指标部分实现

**严重性**: 🟡 中  
**状态**: ⚠️ 部分实现  
**文件**: `src-tauri/src/strategy/indicators.rs`

**当前实现**:
| 指标 | 状态 | 说明 |
|------|------|------|
| SMA | ✅ 已实现 | 简单移动平均 |
| RSI | ✅ 已实现 | 相对强弱指数 |
| EMA | ❌ 未实现 | 指数移动平均 |
| MACD | ❌ 未实现 | 指数平滑移动平均 |
| Bollinger Bands | ❌ 未实现 | 布林带 |
| ATR | ❌ 未实现 | 平均真实波幅 |

**影响**:
- 无法使用 EMA、MACD 等常用指标
- 限制了策略的复杂性
- 与文档声称的 "RSI, SMA, EMA, MACD" 不符

---

## 📊 问题优先级排序

### P0 - 立即修复（阻塞核心功能）

| 问题ID | 描述 | 预计工时 |
|--------|------|----------|
| EXCH-003 | create_order 未实现 | 2天 |
| BACK-001 | 利润计算公式错误 | 1天 |
| BACK-002 | profit_abs 未设置 | 0.5天 |
| STRAT-001 | SimpleStrategy 返回空值 | 1天 |

### P1 - 高优先级（影响用户体验）

| 问题ID | 描述 | 预计工时 |
|--------|------|----------|
| BOT-001 | 硬编码交易对 | 1天 |
| BOT-002 | 买卖逻辑仅打印日志 | 2天 |
| BOT-003 | 缺少多交易对支持 | 2天 |
| EXCH-001 | fetch_balance 未实现 | 1天 |

### P2 - 中优先级（增强功能）

| 问题ID | 描述 | 预计工时 |
|--------|------|----------|
| EXCH-004~006 | 订单管理方法未实现 | 3天 |
| EXCH-007 | WebSocket 支持 | 5天 |
| STRAT-002 | 其他技术指标 | 3天 |

---

## 🔧 修复计划

### 阶段 1: 核心交易功能 (Week 1)

1. **实现订单创建** (EXCH-003)
   - 实现 Binance API 订单创建
   - 添加订单类型支持（market, limit）
   - 测试订单流程

2. **修复回测计算** (BACK-001, BACK-002)
   - 修复利润计算公式
   - 正确设置 profit_abs 和 profit_ratio
   - 验证胜率计算

3. **实现简单策略** (STRAT-001)
   - 实现基于 SMA 的简单买入/卖出信号
   - 添加基本策略配置

### 阶段 2: 交易执行 (Week 2)

1. **实现订单管理** (EXCH-004~006)
   - 实现订单查询
   - 实现订单取消
   - 实现余额获取

2. **修复 Bot 模块** (BOT-001, BOT-002, BOT-003)
   - 从配置读取交易对
   - 实现实际买卖逻辑
   - 添加多交易对循环

### 阶段 3: 高级功能 (Week 3-4)

1. **实现 WebSocket** (EXCH-007)
2. **实现更多技术指标** (STRAT-002)
3. **完善文档和测试**

---

## 📝 检查清单

### 修复前验证

- [ ] 所有 NotImplemented 方法都已实现
- [ ] 回测结果中的 profit_abs 不为空
- [ ] 胜率计算正确
- [ ] SimpleStrategy 产生有效的买卖信号
- [ ] 买卖逻辑实际调用交易所 API

### 修复后验证

- [ ] 编译通过，无警告
- [ ] 单元测试通过
- [ ] 集成测试通过
- [ ] 回测结果合理
- [ ] 文档已更新

---

## 🔗 相关文档

- [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - 项目迁移计划
- [API Reference](docs/api/README.md) - API 文档
- [Development Guide](DEVELOPMENT.md) - 开发指南

---

*最后更新: 2026-01-14*  
*问题追踪: 使用 GitHub Issues 管理*

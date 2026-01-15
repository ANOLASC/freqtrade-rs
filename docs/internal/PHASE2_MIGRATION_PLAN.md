# Freqtrade-rs 二期迁移计划

## 概述
从原 Python 项目 `D:\code\trade\freqtrade` 迁移以下功能到 Rust + Tauri 项目：
1. **风险管理** (Risk Management)
2. **参数优化** (Parameter Optimization)
3. **数据管理** (Data Management)

---

## 模块一：风险管理

### 原项目位置
- `freqtrade/plugins/protections/`

### 核心功能
1. **保护机制接口**
   - 冷却期保护 (Cooldown Period)
   - 低利润对保护 (Low Profit Pairs)
   - 最大回撤保护 (Max Drawdown Protection)
   - 止损保护 (Stoploss Guard)

2. **保护级别**
   - 全局停止
   - 局部停止（单对）

3. **配置参数**
   - 停止持续时间（蜡烛数或分钟）
   - 回顾周期（蜡烛数或分钟）
   - 解锁时间

### 迁移计划

#### 1.1 创建风险管理模块结构
```
src-tauri/src/
├── risk/
│   ├── mod.rs
│   ├── protection.rs       # 保护机制 trait 定义
│   ├── cooldown.rs         # 冷却期保护
│   ├── low_profit.rs       # 低利润对保护
│   ├── max_drawdown.rs     # 最大回撤保护
│   ├── stoploss_guard.rs   # 止损保护
│   └── manager.rs         # 风险管理器
```

#### 1.2 核心类型定义
```rust
// risk/protection.rs
pub struct ProtectionReturn {
    pub lock: bool,
    pub until: DateTime<Utc>,
    pub reason: Option<String>,
    pub lock_side: String,
}

pub trait IProtection: Send + Sync {
    fn name(&self) -> &str;
    fn short_desc(&self) -> String;
    fn global_stop(&self) -> ProtectionReturn;
    fn stop_per_pair(&self, pair: &str, date_now: DateTime<Utc>, 
                     trades: &[Trade]) -> ProtectionReturn;
}
```

#### 1.3 实现各个保护机制

##### 冷却期保护
- 功能：在亏损后停止交易指定时间
- 参数：`lookback_period`, `stop_duration`
- 触发条件：指定时间内的亏损交易数达到阈值

##### 低利润对保护
- 功能：停止交易连续低利润的交易对
- 参数：`lookback_period`, `stop_duration`, `required_profit`
- 触发条件：指定时间内利润低于阈值

##### 最大回撤保护
- 功能：在达到最大回撤时停止交易
- 参数：`max_allowed_drawdown`, `lookback_period`
- 触发条件：回撤达到阈值

##### 止损保护
- 功能：保护止损不被频繁触发
- 参数：`lookback_period`, `stop_duration`
- 触发条件：频繁止损

#### 1.4 风险管理器
```rust
pub struct RiskManager {
    protections: Vec<Box<dyn IProtection>>,
    config: RiskConfig,
}

impl RiskManager {
    pub async fn check_global_stop(&self) -> Option<StopReason>;
    pub async fn check_pair_stop(&self, pair: &str) -> Option<StopReason>;
    pub fn add_protection(&mut self, protection: Box<dyn IProtection>);
}
```

---

## 模块二：参数优化

### 原项目位置
- `freqtrade/optimize/`

### 核心功能
1. **回测缓存**
2. **超参数优化 (Hyperopt)**
3. **优化报告**
4. **参数空间探索**

### 迁移计划

#### 2.1 创建优化模块结构
```
src-tauri/src/
├── optimize/
│   ├── mod.rs
│   ├── hyperopt.rs        # 超参数优化核心
│   ├── optimizer.rs       # 优化器接口
│   ├── space.rs           # 参数空间定义
│   ├── loss_functions.rs  # 损失函数
│   ├── reports.rs         # 优化报告
│   └── cache.rs          # 回测缓存
```

#### 2.2 核心类型定义
```rust
// optimize/space.rs
pub enum HyperoptSpace {
    Buy(String),           // 买入参数
    Sell(String),          // 卖出参数
    Protection(String),     // 保护参数
    Trailing(String),      # 追踪止损参数
    ROI(String),          // ROI 参数
}

pub enum HyperoptValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

// optimize/hyperopt.rs
pub struct HyperoptResult {
    pub epoch: usize,
    pub results: BacktestResult,
    pub params: HashMap<String, HyperoptValue>,
    pub loss: f64,
    pub is_best: bool,
}
```

#### 2.3 优化算法实现

##### 随机搜索
- 简单的随机参数探索
- 适合初步搜索

##### 贝叶斯优化
- 使用历史结果指导搜索
- 更高效的参数空间探索

##### 网格搜索
- 系统性地探索参数空间
- 适合小规模搜索

#### 2.4 损失函数
```rust
// optimize/loss_functions.rs
pub trait LossFunction: Send + Sync {
    fn calculate(&self, result: &BacktestResult) -> f64;
    fn name(&self) -> &str;
}

// 实现的损失函数：
// - Sharpe Ratio
// - Sortino Ratio
// - Calmar Ratio
// - Profit Factor
// - Custom
```

#### 2.5 优化报告
```rust
// optimize/reports.rs
pub struct HyperoptReport {
    pub epochs: Vec<HyperoptResult>,
    pub best_epoch: usize,
    pub best_params: HashMap<String, HyperoptValue>,
    pub metrics: HyperoptMetrics,
}

impl HyperoptReport {
    pub fn generate_csv(&self) -> String;
    pub fn generate_json(&self) -> String;
    pub fn plot_progress(&self) -> Result<Plot>;
}
```

---

## 模块三：数据管理

### 原项目位置
- `freqtrade/data/`

### 核心功能
1. **数据下载和更新**
2. **数据转换**
3. **回测分析**
4. **指标计算**

### 迁移计划

#### 3.1 创建数据管理模块结构
```
src-tauri/src/
├── data/
│   ├── mod.rs
│   ├── downloader.rs      # 数据下载器
│   ├── converter.rs       # 数据格式转换
│   ├── analyzer.rs        # 回测分析
│   ├── metrics.rs         # 指标计算
│   └── manager.rs        # 数据管理器
```

#### 3.2 数据下载器
```rust
// data/downloader.rs
pub struct DataDownloader {
    exchange: Arc<dyn Exchange>,
    config: DataConfig,
}

impl DataDownloader {
    pub async fn download_pair(
        &self,
        pair: &str,
        timeframe: Timeframe,
        since: DateTime<Utc>,
    ) -> Result<Vec<OHLCV>>;
    
    pub async fn update_data(
        &self,
        pairs: &[Pair],
        timeframe: Timeframe,
    ) -> Result<UpdateResult>;
    
    pub async fn download_multiple(
        &self,
        pairs: &[Pair],
        timeframes: &[Timeframe],
    ) -> Result<DownloadProgress>;
}
```

#### 3.3 数据转换器
```rust
// data/converter.rs
pub struct DataConverter;

impl DataConverter {
    pub fn ohlcv_to_dataframe(&self, data: &[OHLCV]) -> DataFrame;
    pub fn resample(&self, data: &[OHLCV], timeframe: Timeframe) -> Vec<OHLCV>;
    pub fn fill_gaps(&self, data: &mut Vec<OHLCV>);
    pub fn validate(&self, data: &[OHLCV]) -> Result<ValidationResult>;
}
```

#### 3.4 回测分析
```rust
// data/analyzer.rs
pub struct BacktestAnalyzer;

impl BacktestAnalyzer {
    pub fn analyze_trades(&self, trades: &[Trade]) -> TradeAnalysis;
    pub fn calculate_cagr(&self, trades: &[Trade]) -> f64;
    pub fn calculate_max_drawdown(&self, trades: &[Trade]) -> f64;
    pub fn calculate_sharpe_ratio(&self, trades: &[Trade]) -> f64;
    pub fn calculate_sortino_ratio(&self, trades: &[Trade]) -> f64;
    pub fn calculate_profit_factor(&self, trades: &[Trade]) -> f64;
    pub fn generate_daily_returns(&self, trades: &[Trade]) -> Vec<DailyReturn>;
}

pub struct TradeAnalysis {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub avg_profit: Decimal,
    pub avg_loss: Decimal,
    pub best_trade: Option<Trade>,
    pub worst_trade: Option<Trade>,
    pub profit_factor: f64,
    pub avg_duration: Duration,
}
```

#### 3.5 数据管理器
```rust
// data/manager.rs
pub struct DataManager {
    downloader: Arc<DataDownloader>,
    converter: Arc<DataConverter>,
    analyzer: Arc<BacktestAnalyzer>,
    repository: Arc<Repository>,
}

impl DataManager {
    pub async fn get_latest_data(
        &self,
        pair: &str,
        timeframe: Timeframe,
        limit: usize,
    ) -> Result<Vec<OHLCV>>;
    
    pub async fn get_data_range(
        &self,
        pair: &str,
        timeframe: Timeframe,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<OHLCV>>;
    
    pub async fn refresh_data(
        &self,
        pairs: &[Pair],
        timeframe: Timeframe,
    ) -> Result<RefreshResult>;
    
    pub async fn export_data(
        &self,
        pair: &str,
        timeframe: Timeframe,
        format: ExportFormat,
    ) -> Result<Vec<u8>>;
}
```

---

## 数据库扩展

### 新增表结构

```sql
-- 保护锁表
CREATE TABLE IF NOT EXISTS protection_locks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pair TEXT NOT NULL,
    protection_name TEXT NOT NULL,
    lock_until TEXT NOT NULL,
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 超参数优化结果表
CREATE TABLE IF NOT EXISTS hyperopt_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    epoch INTEGER NOT NULL,
    strategy TEXT NOT NULL,
    params_json TEXT NOT NULL,
    results_json TEXT NOT NULL,
    loss REAL NOT NULL,
    is_best INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 数据下载历史表
CREATE TABLE IF NOT EXISTS data_downloads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pair TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    candles_count INTEGER NOT NULL,
    download_time TEXT NOT NULL DEFAULT (datetime('now'))
);
```

---

## Tauri Commands 扩展

### 风险管理 Commands
```rust
#[tauri::command]
async fn get_active_protections() -> Result<Vec<ProtectionInfo>>;

#[tauri::command]
async fn add_protection(config: ProtectionConfig) -> Result<()>;

#[tauri::command]
async fn remove_protection(name

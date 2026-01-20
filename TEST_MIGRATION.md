# 测试迁移文档 - freqtrade Python → freqtrade-rs Rust

**生成日期**: 2026-01-20  
**原项目**: freqtrade (Python)  
**目标项目**: freqtrade-rs (Rust)  
**迁移状态**: 进行中

---

## 1. 测试迁移概览

### 1.1 原项目测试统计

| 模块 | 文件 | 行数 | 测试用例数 | 优先级 |
|------|------|------|-----------|--------|
| Persistence | test_persistence.py | 2,895 | 150+ | **P0** |
| FreqtradeBot | test_freqtradebot.py | 5,917 | 200+ | **P0** |
| Exchange | test_exchange.py | 6,660 | 250+ | **P1** |
| Strategy | test_interface.py | 1,058 | 80+ | **P1** |
| **总计** | **4个核心文件** | **16,530** | **680+** | |

### 1.2 测试迁移优先级

```
P0 (立即迁移):
├── Persistence层测试 (数据库操作、Trade模型、Order模型)
├── Trade核心逻辑测试 (开仓、平仓、计算profit)
└── Risk Management测试 (风险控制逻辑)

P1 (下一步迁移):
├── Exchange集成测试 (交易所API模拟)
└── Strategy接口测试 (策略信号、指标)

P2 (后续迁移):
├── FreqtradeBot集成测试 (完整交易流程)
└── RPC和通信测试
```

---

## 2. Persistence层测试迁移 (2,895行)

### 2.1 核心测试用例映射

#### 2.1.1 Trade模型测试 (Line 26-500)

**Python原测试**:
```python
# test_persistence.py:26-47
@pytest.mark.parametrize("is_short", [False, True])
@pytest.mark.usefixtures("init_persistence")
def test_enter_exit_side(fee, is_short):
    entry_side, exit_side = ("sell", "buy") if is_short else ("buy", "sell")
    trade = Trade(
        id=2, pair="ADA/USDT", stake_amount=0.001, open_rate=0.01,
        amount=5, is_open=True, open_date=dt_now(),
        fee_open=fee.return_value, fee_close=fee.return_value,
        exchange="binance", is_short=is_short, leverage=2.0,
        trading_mode=margin,
    )
    assert trade.entry_side == entry_side
    assert trade.exit_side == exit_side
    assert trade.trade_direction == "short" if is_short else "long"
```

**迁移到Rust** (`src-tauri/src/persistence/trade_tests.rs`):
```rust
// 对应: test_enter_exit_side
#[tokio::test]
async fn test_trade_entry_exit_side() {
    let trade = Trade::new(
        pair: "ADA/USDT",
        stake_amount: Decimal::from(1),
        open_rate: Decimal::from_str("0.01").unwrap(),
        amount: Decimal::from(5),
        is_short: false,
        leverage: Decimal::from(2),
        trading_mode: TradingMode::Margin,
    ).await;
    
    assert_eq!(trade.entry_side(), "buy");
    assert_eq!(trade.exit_side(), "sell");
    
    let short_trade = Trade::new(
        pair: "ADA/USDT",
        is_short: true,
        ..trade.clone()
    ).await;
    
    assert_eq!(short_trade.entry_side(), "sell");
    assert_eq!(short_trade.exit_side(), "buy");
}
```

#### 2.1.2 止损位计算测试 (Line 51-176)

**Python原测试**:
```python
# test_persistence.py:51-176 (详细计算逻辑)
def test_set_stop_loss_liquidation(fee):
    trade = Trade(
        id=2, pair="ADA/USDT", stake_amount=60.0,
        open_rate=2.0, amount=30.0, is_open=True,
        fee_open=fee.return_value, fee_close=fee.return_value,
        exchange="binance", is_short=False, leverage=2.0,
        trading_mode=margin,
    )
    trade.set_liquidation_price(0.09)
    assert trade.liquidation_price == 0.09
    assert trade.stop_loss is None
    
    trade.adjust_stop_loss(2.0, 0.2, True)
    assert trade.liquidation_price == 0.09
    assert trade.stop_loss == 1.8
    assert trade.initial_stop_loss == 1.8
```

**迁移到Rust**:
```rust
// 对应: test_set_stop_loss_liquidation
#[tokio::test]
async fn test_trade_stop_loss_liquidation() {
    let trade = Trade::builder()
        .pair("ADA/USDT")
        .stake_amount(Decimal::from(60))
        .open_rate(Decimal::from_str("2.0").unwrap())
        .amount(Decimal::from(30))
        .is_short(false)
        .leverage(Decimal::from(2))
        .trading_mode(TradingMode::Margin)
        .build();
    
    // 设置清算价格
    trade.set_liquidation_price(Decimal::from_str("0.09").unwrap()).await;
    assert_eq!(trade.liquidation_price, Some(Decimal::from_str("0.09").unwrap()));
    assert!(trade.stop_loss.is_none());
    
    // 调整止损位
    trade.adjust_stop_loss(
        Decimal::from_str("2.0").unwrap(),
        Decimal::from(-0.2),
        true
    ).await;
    
    assert_eq!(trade.liquidation_price, Some(Decimal::from_str("0.09").unwrap()));
    assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.8").unwrap()));
    assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.8").unwrap()));
}
```

### 2.2 利息计算测试迁移 (Line 177-279)

**Python原测试参数化**:
```python
# test_persistence.py:177-206
@pytest.mark.parametrize(
    "exchange,is_short,lev,minutes,rate,interest,trading_mode",
    [
        ("binance", False, 3, 10, 0.0005, round(0.0008333333333333334, 8), margin),
        ("binance", True, 3, 10, 0.0005, 0.000625, margin),
        ("kraken", False, 3, 10, 0.0005, 0.040, margin),
        # ... 更多组合
    ],
)
```

**迁移到Rust使用proptest**:
```rust
// 对应: test_interest
proptest! {
    #[tokio::test]
    async fn test_trade_interest(
        exchange in prop_one_of![Exchange::Binance, Exchange::Kraken],
        is_short in bool::ANY,
        lev in 1..=5u32,
        minutes in 1..=300u32,
    ) {
        let rate = Decimal::from_str("0.0005");
        let interest_rate = Decimal::from_str("0.0005");
        
        let trade = Trade::builder()
            .pair("ADA/USDT")
            .stake_amount(Decimal::from(20))
            .amount(Decimal::from(30))
            .open_rate(Decimal::from(2))
            .exchange(exchange)
            .is_short(is_short)
            .leverage(Decimal::from(lev))
            .interest_rate(interest_rate)
            .build();
        
        let calculated_interest = trade.calculate_interest(minutes).await;
        
        // 验证Binance和Kraken的利息计算差异
        let expected_interest = match exchange {
            Exchange::Binance => {
                // Binance: 1/24 24hr_periods
                let periods = Decimal::from(minutes) / Decimal::from(24 * 60);
                let borrowed = if is_short {
                    Decimal::from(30) * (Decimal::from(lev) - Decimal::from(1))
                } else {
                    Decimal::from(40)
                };
                borrowed * interest_rate * periods
            },
            Exchange::Kraken => {
                // Kraken: (1 + 1) 4hr_periods
                let periods = Decimal::from(1 + (minutes / 240));
                let borrowed = if is_short {
                    Decimal::from(30) * (Decimal::from(lev) - Decimal::from(1))
                } else {
                    Decimal::from(40)
                };
                borrowed * interest_rate * periods
            },
        };
        
        assert_relative_eq!(calculated_interest, expected_interest, max_relative = 0.0001);
    }
}
```

### 2.3 订单更新测试迁移 (Line 373-551)

**Python原测试**:
```python
# test_persistence.py:373-520
@pytest.mark.usefixtures("init_persistence")
def test_update_limit_order(
    fee, caplog, limit_buy_order_usdt, limit_sell_order_usdt,
    time_machine, is_short, open_rate, close_rate, lev, profit,
    trading_mode,
):
    """测试限价单更新和成交"""
    time_machine.move_to("2022-03-31 20:45:00 +00:00")
    
    enter_order = limit_sell_order_usdt if is_short else limit_buy_order_usdt
    exit_order = limit_buy_order_usdt if is_short else limit_sell_order_usdt
    
    trade = Trade(...)  // 创建交易
    oobj = Order.parse_from_ccxt_object(enter_order, "ADA/USDT", entry_side)
    trade.orders.append(oobj)
    trade.update_trade(oobj)
    
    // 验证更新后的状态
    assert not trade.has_open_orders
    assert trade.open_rate == open_rate
```

**迁移到Rust**:
```rust
// 对应: test_update_limit_order
#[tokio::test]
async fn test_trade_update_limit_order() {
    let mut time = TimeMachine::set("2022-03-31T20:45:00Z");
    
    // 创建买入订单
    let enter_order = Order::from_ccxt_object(
        &limit_buy_order_usdt,
        "ADA/USDT",
        OrderSide::Buy,
    ).await;
    
    // 创建交易
    let trade = Trade::builder()
        .pair("ADA/USDT")
        .stake_amount(Decimal::from(60))
        .open_rate(Decimal::from_str("2.0").unwrap())
        .amount(Decimal::from(30))
        .exchange(Exchange::Binance)
        .is_short(false)
        .leverage(Decimal::from(1))
        .trading_mode(TradingMode::Margin)
        .build();
    
    // 添加订单并更新交易
    trade.orders.write().await.push(enter_order.clone());
    trade.update_trade(&enter_order).await;
    
    // 验证结果
    assert!(!trade.has_open_orders());
    assert_eq!(trade.open_rate, Decimal::from_str("2.0").unwrap());
    assert!(trade.close_profit.is_none());
    assert!(trade.close_date.is_none());
    
    // 创建卖出订单
    let exit_order = Order::from_ccxt_object(
        &limit_sell_order_usdt,
        "ADA/USDT",
        OrderSide::Sell,
    ).await;
    
    time.advance(Duration::hours(1));
    
    trade.orders.write().await.push(exit_order.clone());
    trade.update_trade(&exit_order).await;
    
    // 验证平仓结果
    assert!(!trade.has_open_orders());
    assert_eq!(trade.close_rate, Some(Decimal::from_str("2.2").unwrap()));
    assert_relative_eq!(
        trade.close_profit.unwrap(),
        Decimal::from_str("0.09451372").unwrap(),
        max_relative = 0.0001
    );
    assert!(trade.close_date.is_some());
}
```

### 2.4 交易开仓价值计算测试 (Line 776-830)

**Python原测试**:
```python
# test_persistence.py:776-830
@pytest.mark.parametrize("exchange", ["binance", "kraken"])
@pytest.mark.parametrize("trading_mode", [spot, margin, futures])
@pytest.mark.parametrize("lev", [1, 3])
@pytest.mark.parametrize("is_short,fee_rate,result", [...])
@pytest.mark.usefixtures("init_persistence")
def test_calc_open_trade_value(limit_buy_order_usdt, ...):
    """测试开仓价值计算"""
    trade = Trade(
        pair="ADA/USDT", stake_amount=60.0, amount=30.0,
        open_rate=2.0, fee_open=fee_rate, fee_close=fee_rate,
        exchange=exchange, leverage=lev, is_short=is_short,
        trading_mode=trading_mode,
    )
    oobj = Order.parse_from_ccxt_object(limit_buy_order_usdt, ...)
    trade.update_trade(oobj)
    
    assert trade._calc_open_trade_value(trade.amount, trade.open_rate) == result
```

**迁移到Rust**:
```rust
// 对应: test_calc_open_trade_value
#[tokio::test]
async fn test_trade_calc_open_value() {
    // 测试用例: (exchange, is_short, lev, fee_rate, result)
    let test_cases = vec![
        (Exchange::Binance, false, 1, Decimal::from_str("0.0025"), Decimal::from_str("60.15").unwrap()),
        (Exchange::Binance, false, 3, Decimal::from_str("0.0025"), Decimal::from_str("60.15").unwrap()),
        (Exchange::Binance, true, 1, Decimal::from_str("0.0025"), Decimal::from_str("59.85").unwrap()),
        (Exchange::Binance, true, 3, Decimal::from_str("0.0025"), Decimal::from_str("59.85").unwrap()),
        // Kraken用例
        (Exchange::Kraken, false, 1, Decimal::from_str("0.0025"), Decimal::from_str("60.15").unwrap()),
        (Exchange::Kraken, true, 1, Decimal::from_str("0.0025"), Decimal::from_str("59.85").unwrap()),
    ];
    
    for (exchange, is_short, lev, fee_rate, expected) in test_cases {
        let trade = Trade::builder()
            .pair("ADA/USDT")
            .stake_amount(Decimal::from(60))
            .amount(Decimal::from(30))
            .open_rate(Decimal::from_str("2.0").unwrap())
            .fee_open(fee_rate)
            .fee_close(fee_rate)
            .exchange(exchange)
            .is_short(is_short)
            .leverage(Decimal::from(lev))
            .trading_mode(TradingMode::Margin)
            .build();
        
        let open_value = trade.calc_open_trade_value().await;
        assert_relative_eq!(open_value, expected, max_relative = 0.0001);
    }
}
```

---

## 3. 收益计算测试迁移 (Line 831-1204)

### 3.1 完整参数化测试映射

**Python原测试覆盖**:
- 交易所: Binance, Kraken
- 交易模式: Spot, Margin, Futures
- 杠杆: 1x, 3x, 5x
- 多空: Long, Short
- 费率: 0.25%, 0.3%
- 资金费率: -1, 0, 1

**总测试组合**: 120+ 个参数化测试用例

**迁移到Rust**:
```rust
// 对应: test_calc_profit 和 test_calc_close_trade_price
#[derive(Debug, Clone)]
struct ProfitTestCase {
    exchange: Exchange,
    is_short: bool,
    lev: u32,
    open_rate: Decimal,
    close_rate: Decimal,
    fee_rate: Decimal,
    expected_profit: Decimal,
    expected_profit_ratio: Decimal,
    trading_mode: TradingMode,
    funding_fees: i32,
}

impl ProfitTestCase {
    fn test_cases() -> Vec<Self> {
        vec![
            // Binance Spot Long 1x
            Self {
                exchange: Exchange::Binance,
                is_short: false,
                lev: 1,
                open_rate: Decimal::from_str("2.0").unwrap(),
                close_rate: Decimal::from_str("2.1").unwrap(),
                fee_rate: Decimal::from_str("0.0025").unwrap(),
                expected_profit: Decimal::from_str("2.6925").unwrap(),
                expected_profit_ratio: Decimal::from_str("0.044763092").unwrap(),
                trading_mode: TradingMode::Spot,
                funding_fees: 0,
            },
            // Binance Spot Short 1x
            Self {
                exchange: Exchange::Binance,
                is_short: true,
                lev: 1,
                open_rate: Decimal::from_str("2.2 close_rate: Decimal").unwrap(),
               ::from_str("2.1").unwrap(),
                fee_rate: Decimal::from_str("0.0025").unwrap(),
                expected_profit: Decimal::from_str("-3.3088157").unwrap(),
                expected_profit_ratio: Decimal::from_str("-0.055285142").unwrap(),
                trading_mode: TradingMode::Margin,
                funding_fees: 0,
            },
            // 更多测试用例...
        ]
    }
}

#[tokio::test]
async fn test_profit_calculation_comprehensive() {
    for test_case in ProfitTestCase::test_cases() {
        let trade = Trade::builder()
            .pair("ADA/USDT")
            .stake_amount(Decimal::from(60))
            .amount(Decimal::from(30) * Decimal::from(test_case.lev))
            .open_rate(test_case.open_rate)
            .fee_open(test_case.fee_rate)
            .fee_close(test_case.fee_rate)
            .exchange(test_case.exchange)
            .is_short(test_case.is_short)
            .leverage(Decimal::from(test_case.lev))
            .trading_mode(test_case.trading_mode)
            .funding_fees(test_case.funding_fees)
            .build();
        
        let profit_result = trade
            .calculate_profit(test_case.close_rate)
            .await;
        
        assert_relative_eq!(
            profit_result.profit_abs,
            test_case.expected_profit,
            max_relative = 0.0001
        );
        assert_relative_eq!(
            profit_result.profit_ratio,
            test_case.expected_profit_ratio,
            max_relative = 0.0001
        );
    }
}
```

---

## 4. 止损调整测试迁移 (Line 1206-1300)

**Python原测试**:
```python
def test_adjust_stop_loss(fee):
    trade = Trade(..., open_rate=1, max_rate=1)
    
    trade.adjust_stop_loss(trade.open_rate, 0.05, True)
    assert trade.stop_loss == 0.95
    assert trade.stop_loss_pct == -0.05
    assert trade.initial_stop_loss == 0.95
    
    trade.adjust_stop_loss(0.96, 0.05)
    # 验证止损不移动（当前价格低于最高价）
    assert trade.stop_loss == 0.95
```

**迁移到Rust**:
```rust
// 对应: test_adjust_stop_loss
#[tokio::test]
async fn test_trade_adjust_stop_loss() {
    let trade = Trade::builder()
        .pair("ADA/USDT")
        .stake_amount(Decimal::from(30))
        .amount(Decimal::from(30))
        .open_rate(Decimal::from(1))
        .exchange(Exchange::Binance)
        .build();
    
    // 初始调整 - 设置止损
    trade.adjust_stop_loss(
        Decimal::from(1),
        Decimal::from(-0.05),
        true
    ).await;
    
    assert_eq!(trade.stop_loss, Some(Decimal::from_str("0.95").unwrap()));
    assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.05)));
    assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("0.95").unwrap()));
    assert_eq!(trade.initial_stop_loss_pct, Some(Decimal::from(-0.05)));
    
    // 尝试用更高的价格调整 - 不应该移动止损
    trade.adjust_stop_loss(
        Decimal::from_str("0.96").unwrap(),
        Decimal::from(-0.05),
        false
    ).await;
    
    assert_eq!(trade.stop_loss, Some(Decimal::from_str("0.95").unwrap()));
    
    // 用更高的价格调整止盈 - 应该移动止损
    trade.adjust_stop_loss(
        Decimal::from_str("1.3").unwrap(),
        Decimal::from(-0.1),
        false
    ).await;
    
    assert_relative_eq!(
        trade.stop_loss.unwrap(),
        Decimal::from_str("1.17").unwrap(),
        max_relative = 0.0001
    );
}
```

---

## 5. 数据库迁移测试迁移 (test_migrations.py)

### 5.1 迁移测试用例 (Line 74-311)

**Python原测试覆盖**:
- 从旧格式迁移到新格式
- 备份表处理
- 订单表迁移
- PairLock迁移

**迁移到Rust**:
```rust
// 对应: test_migrate
#[tokio::test]
async fn test_database_migration() {
    // 创建模拟的旧格式数据库
    let old_schema = r#"
        CREATE TABLE IF NOT EXISTS "trades" (
            id INTEGER NOT NULL,
            exchange VARCHAR NOT NULL,
            pair VARCHAR NOT NULL,
            is_open BOOLEAN NOT NULL,
            fee FLOAT NOT NULL,
            open_rate FLOAT,
            close_rate FLOAT,
            close_profit FLOAT,
            stake_amount FLOAT NOT NULL,
            amount FLOAT,
            open_date DATETIME NOT NULL,
            close_date DATETIME,
            open_order_id VARCHAR,
            stop_loss FLOAT,
            initial_stop_loss FLOAT,
            max_rate FLOAT,
            sell_reason VARCHAR,
            strategy VARCHAR,
            ticker_interval INTEGER,
            stoploss_order_id VARCHAR,
            PRIMARY KEY (id)
        );
    "#;
    
    let db = Database::new().await;
    db.execute(old_schema).await;
    
    // 插入测试数据
    db.execute("INSERT INTO trades ...").await;
    
    // 执行迁移
    let migrator = DatabaseMigrator::new();
    migrator.migrate(&db).await;
    
    // 验证迁移结果
    let trades = db.find_all_trades().await;
    assert_eq!(trades.len(), 1);
    
    let trade = trades.first().unwrap();
    assert_eq!(trade.pair, "ETC/BTC");
    assert_eq!(trade.amount, Decimal::from_str("103.223").unwrap());
    assert!(trade.is_open);
    
    // 验证订单也迁移了
    let orders = db.find_orders_by_trade_id(trade.id).await;
    assert_eq!(orders.len(), 4);
}
```

---

## 6. 测试文件结构

### 6.1 推荐的Rust测试文件结构

```
src-tauri/src/
├── persistence/
│   ├── mod.rs
│   ├── trade.rs           # Trade模型实现
│   ├── order.rs           # Order模型实现
│   ├── repository.rs      # 数据访问层
│   └── tests/
│       ├── mod.rs         # 测试模块入口
│       ├── trade_tests.rs # Trade模型测试 (2000+ 行)
│       ├── order_tests.rs # Order模型测试 (500+ 行)
│       └── migration_tests.rs # 迁移测试 (400+ 行)
├── risk/
│   ├── mod.rs
│   ├── manager.rs
│   └── tests/
│       ├── mod.rs
│       └── risk_tests.rs  # 风险管理测试 (1500+ 行)
├── exchange/
│   ├── mod.rs
│   ├── binance.rs
│   └── tests/
│       ├── mod.rs
│       └── exchange_tests.rs # 交易所测试 (2500+ 行)
└── strategy/
    ├── mod.rs
    ├── indicators.rs
    └── tests/
        ├── mod.rs
        └── strategy_tests.rs # 策略测试 (800+ 行)
```

### 6.2 测试夹具 (Fixtures)

**Python夹具** (conftest.py):
```python
@pytest.fixture
def fee():
    return MagicMock(return_value=0.0025)

@pytest.fixture
def limit_buy_order_usdt():
    return {
        "id": "12345",
        "symbol": "ADA/USDT",
        "status": "closed",
        "side": "buy",
        "type": "limit",
        "price": 2.0,
        "amount": 30.0,
        "filled": 30.0,
        "cost": 60.0,
    }
```

**Rust对应实现**:
```rust
// src-tauri/src/tests/fixtures/mod.rs

pub struct TestFixtures;

impl TestFixtures {
    pub fn fee() -> Decimal {
        Decimal::from_str("0.0025").unwrap()
    }
    
    pub fn limit_buy_order_usdt() -> OrderResponse {
        OrderResponse {
            id: "12345".to_string(),
            symbol: "ADA/USDT".to_string(),
            status: OrderStatus::Closed,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Decimal::from_str("2.0").unwrap(),
            amount: Decimal::from_str("30.0").unwrap(),
            filled: Decimal::from_str("30.0").unwrap(),
            cost: Decimal::from_str("60.0").unwrap(),
            remaining: Decimal::from(0),
            average: Decimal::from_str("2.0").unwrap(),
        }
    }
    
    pub fn limit_sell_order_usdt() -> OrderResponse {
        // ...
    }
}
```

---

## 7. 迁移工具和依赖

### 7.1 Rust测试依赖

```toml
# Cargo.toml
[dev-dependencies]
tokio = { version = "1.0", features = ["test-util"] }
proptest = "1.0"
quickcheck = "1.0"
quickcheck_macros = "1.0"
rstest = "0.18"
time-machine = "0.3"  # 时间模拟
assert_matches = "0.1"
approx = "0.5"  # 浮点数比较

[dev-dependencies.sqlx]
version = "0.7"
features = ["sqlite", "runtime-tokio"]
```

### 7.2 测试宏和工具

```rust
// src-tauri/src/tests/macros.rs

#[macro_export]
macro_rules! assert_rel_eq {
    ($left:expr, $right:expr) => {
        assert_relative_eq!($left, $right, max_relative = 0.0001)
    };
    ($left:expr, $right:expr, $max:expr) => {
        assert_relative_eq!($left, $right, max_relative = $max)
    };
}

#[macro_export]
macro_rules! parameterize {
    ($test_func:ident, $cases:expr) => {
        $cases.into_iter().for_each(|$case| {
            $test_func($case).await;
        });
    };
}
```

---

## 8. 持续集成配置

### 8.1 测试命令

```bash
# 运行所有持久化测试
cd src-tauri
cargo test --lib persistence::tests

# 运行单个测试文件
cargo test --lib --test trade_tests

# 运行带日志的测试
RUST_LOG=debug cargo test test_trade_interest -- --nocapture

# 运行性能测试
cargo test --lib --release -- test_profit_calculation
```

### 8.2 CI配置示例

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: 1.70+
          
      - name: Run Persistence Tests
        run: |
          cd src-tauri
          cargo test --lib persistence::tests -- --test-threads=4
          
      - name: Run Trade Logic Tests
        run: |
          cd src-tauri
          cargo test --lib trade_tests -- --test-threads=4
          
      - name: Run Integration Tests
        run: |
          cd src-tauri
          cargo test --test integration_tests
```

---

## 9. 迁移进度追踪

### 9.1 已完成 ✅

- [x] 分析原项目测试结构
- [x] 识别可迁移测试用例
- [x] 设计Rust测试架构
- [x] 创建Persistence层测试模板

### 9.2 进行中 🔄

- [ ] 迁移Trade模型核心测试 (Line 26-500)
- [ ] 迁移止损和清算测试 (Line 51-176)
- [ ] 迁移利息计算测试 (Line 177-279)

### 9.3 待完成 ⏳

- [ ] 迁移订单更新测试 (Line 373-551)
- [ ] 迁移收益计算测试 (Line 831-1204)
- [ ] 迁移数据库迁移测试 (test_migrations.py)
- [ ] 迁移Exchange模块测试
- [ ] 迁移Strategy模块测试
- [ ] 迁移FreqtradeBot集成测试
- [ ] 创建完整的测试套件

---

## 10. 关键迁移注意事项

### 10.1 类型转换

| Python | Rust | 注意事项 |
|--------|------|---------|
| `float` | `Decimal` | 金融计算必须用Decimal |
| `datetime` | `DateTime<Utc>` | 使用chrono |
| `MagicMock` | `Arc<RwLock<T>>` | 异步模拟 |
| `pytest.raises` | `assert!(panic!)` | 错误处理测试 |

### 10.2 异步转换

Python:
```python
async def test_trade_update():
    trade = Trade(...)
    await trade.update_trade(order)  # 使用asyncio
```

Rust:
```rust
#[tokio::test]
async fn test_trade_update() {
    let trade = Trade::new().await;
    trade.update_trade(&order).await;
}
```

### 10.3 状态管理

Python:
```python
# 共享状态通过fixture
@pytest.fixture
def trade(db):
    return Trade.open(...)
```

Rust:
```rust
// 每个测试使用独立的数据库实例
async fn setup_test_db() -> Database {
    Database::new_in_memory().await
}
```

---

## 11. 参考资源

- **原项目测试文件**: `/d/code/trade/freqtrade/tests/`
- **原项目Persistence测试**: `test_persistence.py` (2,895行)
- **原项目迁移测试**: `test_migrations.py` (450+行)
- **Rust测试最佳实践**: https://doc.rust-lang.org/book/ch11-00-testing.html

---

**文档版本**: 1.0  
**最后更新**: 2026-01-20  
**维护者**: AI Assistant

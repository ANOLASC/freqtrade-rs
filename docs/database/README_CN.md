# 数据库模式

> freqtrade-rs SQLite 数据库模式的完整参考。

## 概述

freqtrade-rs 使用 SQLite 作为其主要数据库，通过 SQLx 进行类型安全的查询。数据库存储交易数据、配置和历史记录。

## 数据库位置

- **开发**：`user_data/freqtrade.db`
- **生产**：通过 `config/default.toml` 配置

## 数据库初始化

数据库在首次运行时自动初始化：
```bash
cargo run
```

迁移从 `migrations/001_initial.sql` 应用。

---

## 表参考

### trades

存储所有交易记录（未平仓和平仓的）。

| 列 | 类型 | 可空 | 默认值 | 描述 |
|--------|------|----------|---------|-------------|
| id | TEXT | 否 | - | 唯一交易标识符（UUID） |
| pair | TEXT | 否 | - | 交易对（例如 "BTC/USDT"） |
| is_open | INTEGER | 否 | - | 1 表示未平仓，0 表示平仓 |
| exchange | TEXT | 否 | - | 交易所名称 |
| open_rate | TEXT | 否 | - | 入场价格（作为字符串存储以保持 Decimal 精度） |
| open_date | TEXT | 否 | - | 入场时间戳（ISO 8601） |
| close_rate | TEXT | 是 | NULL | 出场价格 |
| close_date | TEXT | 是 | NULL | 出场时间戳 |
| amount | TEXT | 否 | - | 交易数量 |
| stake_amount | TEXT | 否 | - |  stake 数量 |
| strategy | TEXT | 否 | - | 策略名称 |
| timeframe | TEXT | 否 | - | 时间周期 |
| stop_loss | TEXT | 是 | NULL | 止损价格 |
| take_profit | TEXT | 是 | NULL | 止盈价格 |
| exit_reason | TEXT | 是 | NULL | 退出类型 |
| profit_abs | TEXT | 是 | NULL | 绝对利润 |
| profit_ratio | TEXT | 是 | NULL | 利润比例 |
| created_at | TEXT | 否 | datetime('now') | 记录创建时间 |
| updated_at | TEXT | 否 | datetime('now') | 最后更新时间 |

**索引**：
- `idx_trades_is_open` - 用于查询未平仓交易
- `idx_trades_pair` - 用于按交易对筛选
- `idx_trades_open_date` - 用于基于时间的查询

**示例查询**：
```sql
SELECT * FROM trades WHERE is_open = 1 ORDER BY open_date DESC;
```

---

### klines

存储 OHLCV（开盘-最高-最低-收盘-成交量）K线数据。

| 列 | 类型 | 可空 | 默认值 | 描述 |
|--------|------|----------|---------|-------------|
| pair | TEXT | 否 | - | 交易对 |
| timeframe | TEXT | 否 | - | K线时间周期 |
| open_time | TEXT | 否 | - | 开盘时间戳（ISO 8601） |
| open | TEXT | 否 | - | 开盘价 |
| high | TEXT | 否 | - | 最高价 |
| low | TEXT | 否 | - | 最低价 |
| close | TEXT | 否 | - | 收盘价 |
| volume | TEXT | 否 | - | 交易量 |
| close_time | TEXT | 否 | - | 收盘时间戳 |

**主键**：(pair, timeframe, open_time)

**索引**：
- `idx_klines_pair_timeframe` - 用于高效的交易对/时间周期查询
- `idx_klines_open_time` - 用于基于时间的查询

**示例查询**：
```sql
SELECT * FROM klines
WHERE pair = 'BTC/USDT' AND timeframe = '1h'
ORDER BY open_time DESC
LIMIT 100;
```

---

### backtest_results

存储回测结果用于分析和比较。

| 列 | 类型 | 可空 | 默认值 | 描述 |
|--------|------|----------|---------|-------------|
| id | INTEGER | 否 | - | 主键（AUTOINCREMENT） |
| strategy | TEXT | 否 | - | 策略名称 |
| pair | TEXT | 否 | - | 交易对 |
| timeframe | TEXT | 否 | - | 使用的时间周期 |
| start_date | TEXT | 否 | - | 回测开始日期 |
| end_date | TEXT | 否 | - | 回测结束日期 |
| total_trades | INTEGER | 否 | - | 交易总数 |
| winning_trades | INTEGER | 否 | - | 盈利交易数 |
| losing_trades | INTEGER | 否 | - | 亏损交易数 |
| win_rate | REAL | 否 | - | 胜率（0-1） |
| total_profit | TEXT | 否 | - | 总利润/亏损 |
| max_drawdown | REAL | 否 | - | 最大回撤 |
| sharpe_ratio | REAL | 否 | - | 夏普比率 |
| profit_factor | REAL | 否 | - | 盈亏比 |
| avg_profit | TEXT | 否 | - | 平均盈利 |
| avg_loss | TEXT | 否 | - | 平均亏损 |
| config | TEXT | 否 | - | 完整结果（JSON格式） |
| created_at | TEXT | 否 | datetime('now') | 记录创建时间 |

**索引**：
- `idx_backtest_results_strategy` - 用于按策略筛选
- `idx_backtest_results_created_at` - 用于按日期排序

---

### protection_locks

存储来自风险管理的活动保护锁。

| 列 | 类型 | 可空 | 默认值 | 描述 |
|--------|------|----------|---------|-------------|
| id | INTEGER | 否 | - | 主键（AUTOINCREMENT） |
| pair | TEXT | 是 | NULL | 交易对（NULL = 全局） |
| protection_name | TEXT | 否 | - | 保护机制名称 |
| lock_until | TEXT | 否 | - | 解锁时间戳（ISO 8601） |
| reason | TEXT | 是 | NULL | 锁定原因 |
| created_at | TEXT | 否 | datetime('now') | 记录创建时间 |

**索引**：
- `idx_protection_locks_pair` - 用于特定交易对锁
- `idx_protection_locks_lock_until` - 用于查找过期的锁

**示例查询**：
```sql
-- 查找所有活动的锁
SELECT * FROM protection_locks WHERE lock_until > datetime('now');

-- 查找特定交易对的锁
SELECT * FROM protection_locks WHERE pair = 'BTC/USDT';
```

---

### hyperopt_results

存储超参数优化结果。

| 列 | 类型 | 可空 | 默认值 | 描述 |
|--------|------|----------|---------|-------------|
| id | INTEGER | 否 | - | 主键（AUTOINCREMENT） |
| epoch | INTEGER | 否 | - | 轮次编号 |
| strategy | TEXT | 否 | - | 策略名称 |
| params_json | TEXT | 否 | - | 参数（JSON格式） |
| results_json | TEXT | 否 | - | 结果（JSON格式） |
| loss | REAL | 否 | - | 损失函数值 |
| is_best | INTEGER | 否 | 0 | 1 表示这是最佳结果 |
| created_at | TEXT | 否 | datetime('now') | 记录创建时间 |

**索引**：
- `idx_hyperopt_results_strategy` - 用于按策略筛选
- `idx_hyperopt_results_is_best` - 用于查找最佳结果

---

### data_downloads

存储数据下载历史记录用于跟踪和审计。

| 列 | 类型 | 可空 | 默认值 | 描述 |
|--------|------|----------|---------|-------------|
| id | INTEGER | 否 | - | 主键（AUTOINCREMENT） |
| pair | TEXT | 否 | - | 交易对 |
| timeframe | TEXT | 否 | - | 时间周期 |
| start_date | TEXT | 否 | - | 下载开始日期 |
| end_date | TEXT | 否 | - | 下载结束日期 |
| candles_count | INTEGER | 否 | - | 下载的K线数量 |
| download_time | TEXT | 否 | datetime('now') | 下载时间戳 |

**索引**：
- `idx_data_downloads_pair` - 用于特定交易对历史
- `idx_data_downloads_download_time` - 用于基于时间的查询

---

## 实体关系图

```
┌─────────────────────────────────────────────────────────────┐
│                        trades                                │
│  id (PK) ──► pair (FK) ──► exchange                         │
│  open_date ──► klines.open_time                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      klines                                  │
│  pair (PK, FK) ──► exchanges.name                           │
│  timeframe (PK)                                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   backtest_results                           │
│  id (PK) ──► strategy (FK)                                  │
│  pair (FK) ──► trades.pair                                  │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                   protection_locks                           │
│  id (PK) ──► pair (FK) ──► trades.pair                      │
└─────────────────────────────────────────────────────────────┘
```

---

## 数据类型

### 日期时间处理

所有时间戳都存储为 ISO 8601 字符串：
```
2024-01-15T10:30:00Z
```

**解析示例**：
```rust
use chrono::{DateTime, Utc, TimeZone};

let timestamp: DateTime<Utc> = DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
    .unwrap()
    .with_timezone(&Utc);
```

### Decimal 处理

货币值存储为字符串以保持精度：
```rust
// 存储
"50000.12345678"

// 检索
let value: Decimal = row.get::<&str, _>("open_rate").parse().unwrap();
```

### UUID 处理

交易 ID 使用 UUID v4：
```rust
use uuid::Uuid;

let trade_id = Uuid::new_v4();
// 存储: "550e8400-e29b-41d4-a716-446655440000"
```

---

## 迁移

### 迁移文件

| 文件 | 描述 |
|------|-------------|
| `migrations/001_initial.sql` | 初始模式创建 |

### 运行迁移

迁移在启动时自动应用：

```rust
// 在 repository.rs 中
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    let migration_sql = include_str!("../../../migrations/001_initial.sql");
    for statement in migration_sql.split(";").map(|s| s.trim()).filter(|s| !s.is_empty()) {
        sqlx::query(statement).execute(pool).await?;
    }
    Ok(())
}
```

### 创建新迁移

1. 创建新的 SQL 文件：`migrations/002_add_feature.sql`
2. 使用 SQLx CLI 应用更改：
   ```bash
   sqlx migrate add add_feature
   ```
3. 测试迁移：
   ```bash
   cargo run  # 应该自动应用
   ```

---

## 性能考虑

### 索引

自动创建以下索引：
- `idx_trades_is_open` - 更快的未平仓交易查询
- `idx_trades_pair` - 更快的交易对筛选
- `idx_klines_pair_timeframe` - 更快的 OHLCV 查询
- `idx_protection_locks_lock_until` - 更快的锁过期检查

### 查询优化

**好**：
```sql
-- 高效使用索引
SELECT * FROM trades WHERE pair = 'BTC/USDT' AND is_open = 1;

-- 基于时间的查询
SELECT * FROM klines WHERE pair = 'BTC/USDT' 
  AND open_time >= '2024-01-01' 
  AND open_time <= '2024-01-31';
```

**避免**：
```sql
-- 索引列上的函数阻止索引使用
SELECT * FROM trades WHERE upper(pair) = 'BTC/USDT';

-- 前导通配符
SELECT * FROM trades WHERE pair LIKE '%BTC%';
```

### 连接池

SQLx 使用在 `repository.rs` 中配置的连接池：
```rust
let pool = SqlitePool::connect(&db_url).await?;
```

默认池大小适合单用户桌面应用程序。

---

## 备份和恢复

### 手动备份

```bash
# 复制数据库文件
cp user_data/freqtrade.db user_data/freqtrade_backup.db

# 或使用 sqlite3 VACUUM INTO
sqlite3 user_data/freqtrade.db ".backup user_data/freqtrade_backup.db"
```

### 恢复

```bash
# 从备份恢复
cp user_data/freqtrade_backup.db user_data/freqtrade.db
```

---

## 相关文档

- [API 参考](README.md) - 使用交易 API
- [仓库模式](docs/development/ARCHITECTURE.md) - 数据库层架构
- [部署指南](docs/operations/DEPLOYMENT.md) - 生产数据库设置

---

*最后更新：2026-01-14*

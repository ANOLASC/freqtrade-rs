# Database Schema

> Complete reference for the freqtrade-rs SQLite database schema.

## Overview

freqtrade-rs uses SQLite as its primary database, accessed through SQLx for type-safe queries. The database stores trading data, configuration, and historical records.

## Database Location

- **Development**: `user_data/freqtrade.db`
- **Production**: Configured via `config/default.toml`

## Database Initialization

The database is initialized automatically on first run:
```bash
cargo run
```

Migrations are applied from `migrations/001_initial.sql`.

---

## Table Reference

### trades

Stores all trade records (both open and closed).

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| id | TEXT | No | - | Unique trade identifier (UUID) |
| pair | TEXT | No | - | Trading pair (e.g., "BTC/USDT") |
| is_open | INTEGER | No | - | 1 if open, 0 if closed |
| exchange | TEXT | No | - | Exchange name |
| open_rate | TEXT | No | - | Entry price (as string for Decimal) |
| open_date | TEXT | No | - | Entry timestamp (ISO 8601) |
| close_rate | TEXT | Yes | NULL | Exit price |
| close_date | TEXT | Yes | NULL | Exit timestamp |
| amount | TEXT | No | - | Trade amount |
| stake_amount | TEXT | No | - | Stake amount |
| strategy | TEXT | No | - | Strategy name |
| timeframe | TEXT | No | - | Timeframe used |
| stop_loss | TEXT | Yes | NULL | Stop loss price |
| take_profit | TEXT | Yes | NULL | Take profit price |
| exit_reason | TEXT | Yes | NULL | Exit type |
| profit_abs | TEXT | Yes | NULL | Absolute profit |
| profit_ratio | TEXT | Yes | NULL | Profit ratio |
| created_at | TEXT | No | datetime('now') | Record creation time |
| updated_at | TEXT | No | datetime('now') | Last update time |

**Indexes**:
- `idx_trades_is_open` - For querying open trades
- `idx_trades_pair` - For filtering by trading pair
- `idx_trades_open_date` - For time-based queries

**Example Query**:
```sql
SELECT * FROM trades WHERE is_open = 1 ORDER BY open_date DESC;
```

---

### klines

Stores OHLCV (Open-High-Low-Close-Volume) candlestick data.

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| pair | TEXT | No | - | Trading pair |
| timeframe | TEXT | No | - | Candle timeframe |
| open_time | TEXT | No | - | Open timestamp (ISO 8601) |
| open | TEXT | No | - | Open price |
| high | TEXT | No | - | High price |
| low | TEXT | No | - | Low price |
| close | TEXT | No | - | Close price |
| volume | TEXT | No | - | Trading volume |
| close_time | TEXT | No | - | Close timestamp |

**Primary Key**: `(pair, timeframe, open_time)`

**Indexes**:
- `idx_klines_pair_timeframe` - For efficient pair/timeframe queries
- `idx_klines_open_time` - For time-based queries

**Example Query**:
```sql
SELECT * FROM klines
WHERE pair = 'BTC/USDT' AND timeframe = '1h'
ORDER BY open_time DESC
LIMIT 100;
```

---

### backtest_results

Stores backtesting results for analysis and comparison.

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| id | INTEGER | No | - | Primary key (AUTOINCREMENT) |
| strategy | TEXT | No | - | Strategy name |
| pair | TEXT | No | - | Trading pair |
| timeframe | TEXT | No | - | Timeframe used |
| start_date | TEXT | No | - | Backtest start date |
| end_date | TEXT | No | - | Backtest end date |
| total_trades | INTEGER | No | - | Total number of trades |
| winning_trades | INTEGER | No | - | Number of winning trades |
| losing_trades | INTEGER | No | - | Number of losing trades |
| win_rate | REAL | No | - | Win rate (0-1) |
| total_profit | TEXT | No | - | Total profit/loss |
| max_drawdown | REAL | No | - | Maximum drawdown |
| sharpe_ratio | REAL | No | - | Sharpe ratio |
| profit_factor | REAL | No | - | Profit factor |
| avg_profit | TEXT | No | - | Average profit |
| avg_loss | TEXT | No | - | Average loss |
| config | TEXT | No | - | Full result as JSON |
| created_at | TEXT | No | datetime('now') | Record creation time |

**Indexes**:
- `idx_backtest_results_strategy` - For filtering by strategy
- `idx_backtest_results_created_at` - For sorting by date

---

### protection_locks

Stores active protection locks from risk management.

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| id | INTEGER | No | - | Primary key (AUTOINCREMENT) |
| pair | TEXT | Yes | NULL | Trading pair (NULL = global) |
| protection_name | TEXT | No | - | Protection mechanism name |
| lock_until | TEXT | No | - | Unlock timestamp (ISO 8601) |
| reason | TEXT | Yes | NULL | Reason for lock |
| created_at | TEXT | No | datetime('now') | Record creation time |

**Indexes**:
- `idx_protection_locks_pair` - For pair-specific locks
- `idx_protection_locks_lock_until` - For finding expired locks

**Example Query**:
```sql
-- Find all active locks
SELECT * FROM protection_locks WHERE lock_until > datetime('now');

-- Find locks for specific pair
SELECT * FROM protection_locks WHERE pair = 'BTC/USDT';
```

---

### hyperopt_results

Stores hyperparameter optimization results.

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| id | INTEGER | No | - | Primary key (AUTOINCREMENT) |
| epoch | INTEGER | No | - | Epoch number |
| strategy | TEXT | No | - | Strategy name |
| params_json | TEXT | No | - | Parameters as JSON |
| results_json | TEXT | No | - | Results as JSON |
| loss | REAL | No | - | Loss function value |
| is_best | INTEGER | No | 0 | 1 if this is the best result |
| created_at | TEXT | No | datetime('now') | Record creation time |

**Indexes**:
- `idx_hyperopt_results_strategy` - For filtering by strategy
- `idx_hyperopt_results_is_best` - For finding best results

---

### data_downloads

Stores data download history for tracking and auditing.

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| id | INTEGER | No | - | Primary key (AUTOINCREMENT) |
| pair | TEXT | No | - | Trading pair |
| timeframe | TEXT | No | - | Timeframe |
| start_date | TEXT | No | - | Download start date |
| end_date | TEXT | No | - | Download end date |
| candles_count | INTEGER | No | - | Number of candles downloaded |
| download_time | TEXT | No | datetime('now') | Download timestamp |

**Indexes**:
- `idx_data_downloads_pair` - For pair-specific history
- `idx_data_downloads_download_time` - For time-based queries

---

## Entity Relationship Diagram

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

## Data Types

### DateTime Handling

All timestamps are stored as ISO 8601 strings:
```
2024-01-15T10:30:00Z
```

**Example Parsing**:
```rust
use chrono::{DateTime, Utc, TimeZone};

let timestamp: DateTime<Utc> = DateTime::parse_from_rfc3339("2024-01-15T10:30:00Z")
    .unwrap()
    .with_timezone(&Utc);
```

### Decimal Handling

Monetary values are stored as strings to preserve precision:
```rust
// Storage
"50000.12345678"

// Retrieval
let value: Decimal = row.get::<&str, _>("open_rate").parse().unwrap();
```

### UUID Handling

Trade IDs use UUID v4:
```rust
use uuid::Uuid;

let trade_id = Uuid::new_v4();
// Storage: "550e8400-e29b-41d4-a716-446655440000"
```

---

## Migrations

### Migration Files

| File | Description |
|------|-------------|
| `migrations/001_initial.sql` | Initial schema creation |

### Running Migrations

Migrations are automatically applied on startup:

```rust
// In repository.rs
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    let migration_sql = include_str!("../../../migrations/001_initial.sql");
    for statement in migration_sql.split(";").map(|s| s.trim()).filter(|s| !s.is_empty()) {
        sqlx::query(statement).execute(pool).await?;
    }
    Ok(())
}
```

### Creating New Migrations

1. Create new SQL file: `migrations/002_add_feature.sql`
2. Apply changes using SQLx CLI:
   ```bash
   sqlx migrate add add_feature
   ```
3. Test migration:
   ```bash
   cargo run  # Should apply automatically
   ```

---

## Performance Considerations

### Indexes

The following indexes are created automatically:
- `idx_trades_is_open` - Faster open trades query
- `idx_trades_pair` - Faster pair filtering
- `idx_klines_pair_timeframe` - Faster OHLCV queries
- `idx_protection_locks_lock_until` - Faster lock expiration checks

### Query Optimization

**Good**:
```sql
-- Uses indexes efficiently
SELECT * FROM trades WHERE pair = 'BTC/USDT' AND is_open = 1;

-- Time-bounded queries
SELECT * FROM klines WHERE pair = 'BTC/USDT' 
  AND open_time >= '2024-01-01' 
  AND open_time <= '2024-01-31';
```

**Avoid**:
```sql
-- Functions on indexed columns prevent index usage
SELECT * FROM trades WHERE upper(pair) = 'BTC/USDT';

-- Leading wildcards
SELECT * FROM trades WHERE pair LIKE '%BTC%';
```

### Connection Pooling

SQLx uses a connection pool configured in `repository.rs`:
```rust
let pool = SqlitePool::connect(&db_url).await?;
```

Default pool size is suitable for single-user desktop application.

---

## Backup and Recovery

### Manual Backup

```bash
# Copy database file
cp user_data/freqtrade.db user_data/freqtrade_backup.db

# Or use sqlite3 VACUUM INTO
sqlite3 user_data/freqtrade.db ".backup user_data/freqtrade_backup.db"
```

### Automated Backup

See `docs/operations/BACKUP.md` for backup automation setup.

### Recovery

```bash
# Restore from backup
cp user_data/freqtrade_backup.db user_data/freqtrade.db
```

---

## Related Documentation

- [API Reference](README.md) - Using trades API
- [Repository Pattern](docs/development/ARCHITECTURE.md) - Database layer architecture
- [Deployment Guide](docs/operations/DEPLOYMENT.md) - Production database setup

---

*Last updated: 2026-01-14*

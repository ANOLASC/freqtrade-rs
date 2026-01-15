# 修复 repository.rs 的步骤

## 在 VS Code 或其他 IDE 中打开项目：
`code D:/code/trade/freqtrade-rs`

## 替换 src-tauri/src/persistence/repository.rs 的内容：

```rust
use crate::error::{AppError, Result};
use crate::types::*;
use chrono::{DateTime, TimeZone, Utc};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Repository {
    pool: Arc<SqlitePool>,
}

impl Repository {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        if let Some(parent) = db_path.as_ref().parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                AppError::Config(format!("Failed to create database directory: {}", e))
            })?;
        }
        let db_url = format!("sqlite:{}", db_path.as_ref().display());
        let pool = SqlitePool::connect(&db_url)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Self::run_migrations(&pool).await?;
        Ok(Self { pool: Arc::new(pool) })
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        let migration_sql = include_str!("../../../migrations/001_initial.sql");
        for statement in migration_sql.split(";").map(|s| s.trim()).filter(|s| !s.is_empty()) {
            sqlx::query(statement).execute(pool).await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn create_trade(&self, trade: &Trade) -> Result<()> {
        let exit_reason = trade.exit_reason.map(|e| e.to_string());
        sqlx::query("INSERT INTO trades (id, pair, is_open, exchange, open_rate, open_date, close_rate, close_date, amount, stake_amount, strategy, timeframe, stop_loss, take_profit, exit_reason, profit_abs, profit_ratio) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(trade.id.to_string()).bind(&trade.pair).bind(trade.is_open as i32)
            .bind(&trade.exchange).bind(trade.open_rate.to_string()).bind(trade.open_date.to_rfc3339())
            .bind(trade.close_rate.map(|v| v.to_string())).bind(trade.close_date.map(|d| d.to_rfc3339()))
            .bind(trade.amount.to_string()).bind(trade.stake_amount.to_string()).bind(&trade.strategy)
            .bind(trade.timeframe.as_str()).bind(trade.stop_loss.map(|v| v.to_string()))
            .bind(trade.take_profit.map(|v| v.to_string())).bind(exit_reason)
            .bind(trade.profit_abs.map(|v| v.to_string())).bind(trade.profit_ratio.map(|v| v.to_string()))
            .execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn get_open_trades(&self) -> Result<Vec<Trade>> {
        let rows = sqlx::query("SELECT * FROM trades WHERE is_open = 1 ORDER BY open_date DESC")
            .fetch_all(&*self.pool).await?;
        rows.iter().map(|row| self.row_to_trade(row)).collect()
    }

    pub async fn get_all_trades(&self) -> Result<Vec<Trade>> {
        let rows = sqlx::query("SELECT * FROM trades ORDER BY open_date DESC")
            .fetch_all(&*self.pool).await?;
        rows.iter().map(|row| self.row_to_trade(row)).collect()
    }

    pub async fn update_trade(&self, trade: &Trade) -> Result<()> {
        let exit_reason = trade.exit_reason.map(|e| e.to_string());
        sqlx::query("UPDATE trades SET close_rate = ?, close_date = ?, stop_loss = ?, take_profit = ?, exit_reason = ?, profit_abs = ?, profit_ratio = ?, is_open = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(trade.close_rate.map(|v| v.to_string())).bind(trade.close_date.map(|d| d.to_rfc3339()))
            .bind(trade.stop_loss.map(|v| v.to_string())).bind(trade.take_profit.map(|v| v.to_string()))
            .bind(exit_reason).bind(trade.profit_abs.map(|v| v.to_string()))
            .bind(trade.profit_ratio.map(|v| v.to_string())).bind(trade.is_open as i32)
            .bind(trade.id.to_string()).execute(&*self.pool).await?;
        Ok(())
    }

    pub async fn save_klines(&self, pair: &str, timeframe: &str, klines: &[OHLCV]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for kline in klines {
            sqlx::query("INSERT OR REPLACE INTO klines (pair, timeframe, open_time, open, high, low, close, volume, close_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(pair).bind(timeframe).bind(kline.timestamp.to_rfc3339())
                .bind(kline.open.to_string()).bind(kline.high.to_string()).bind(kline.low.to_string())
                .bind(kline.close.to_string()).bind(kline.volume.to_string())
                .execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_klines(&self, pair: &str, timeframe: &str, limit: usize) -> Result<Vec<OHLCV>> {
        let rows = sqlx::query("SELECT * FROM klines WHERE pair = ? AND timeframe = ? ORDER BY open_time DESC LIMIT ?")
            .bind(pair).bind(timeframe).bind(limit as i64)
            .fetch_all(&*self.pool).await?;
        rows.iter().map(|row| self.row_to_kline(row)).collect()
    }

    pub async fn save_backtest_result(&self, result: &BacktestResult) -> Result<i64> {
        let row = sqlx::query("INSERT INTO backtest_results (strategy, pair, timeframe, start_date, end_date, total_trades, winning_trades, losing_trades, win_rate, total_profit, max_drawdown, sharpe_ratio, profit_factor, avg_profit, avg_loss, config) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&result.strategy)
            .bind(&result.pair)
            .bind(result.timeframe.as_str())
            .bind(result.start_date.to_rfc3339())
            .bind(result.end_date.to_rfc3339())
            .bind(result.total_trades as i64)
            .bind(result.winning_trades as i64)
            .bind(result.losing_trades as i64)
            .bind(result.win_rate)
            .bind(result.total_profit.to_string())
            .bind(result.max_drawdown)
            .bind(result.sharpe_ratio)
            .bind(result.profit_factor)
            .bind(result.avg_profit.to_string())
            .bind(result.avg_loss.to_string())
            .bind(serde_json::to_string(result).unwrap())
            .execute(&*self.pool).await?;
        Ok(row.last_insert_rowid())
    }

    pub async fn get_backtest_results(&self) -> Result<Vec<BacktestResult>> {
        let rows = sqlx::query("SELECT * FROM backtest_results ORDER BY created_at DESC")
            .fetch_all(&*self.pool).await?;
        rows.iter().map(|row| {
            let config_json: String = row.get("config");
            let result: BacktestResult = serde_json::from_str(&config_json)
                .map_err(|e| AppError::Serialization(e))?;
            Ok(result)
        }).collect()
    }

    fn row_to_trade(&self, row: &Row) -> Result<Trade> {
        Ok(Trade {
            id: Uuid::parse_str(row.get("id")).map_err(|e| AppError::Parse(format!("Invalid UUID: {}", e)))?,
            pair: row.get("pair"),
            is_open: row.get::<i32, _>("is_open") != 0,
            exchange: row.get("exchange"),
            open_rate: row.get::<&str, _>("open_rate").parse().map_err(|e| AppError::Parse(format!("Invalid decimal: {}", e)))?,
            open_date: DateTime::parse_from_rfc3339(row.get("open_date")).map_err(|e| AppError::Parse(format!("Invalid datetime: {}", e)))?.with_timezone(&Utc),
            close_rate: row.get::<Option<&str>, _>("close_rate").map(|s| s.parse().ok()).flatten(),
            close_date: row.get::<Option<&str>, _>("close_date").and_then(|s| DateTime::parse_from_rfc3339(s).ok()).map(|dt| dt.with_timezone(&Utc)),
            amount: row.get::<&str, _>("amount").parse().map_err(|e| AppError::Parse(format!("Invalid decimal: {}", e)))?,
            stake_amount: row.get::<&str, _>("stake_amount").parse().map_err(|e| AppError::Parse(format!("Invalid decimal: {}", e)))?,
            strategy: row.get("strategy"),
            timeframe: match row.get::<&str, _>("timeframe") {
                "1m" => Timeframe::OneMinute, "5m" => Timeframe::FiveMinutes, "15m" => Timeframe::FifteenMinutes,
                "30m" => Timeframe::ThirtyMinutes, "1h" => Timeframe::OneHour, "4h" => Timeframe::FourHours,
                "1d" => Timeframe::OneDay, _ => Timeframe::OneHour,
            },
            stop_loss: row.get::<Option<&str>, _>("stop_loss").map(|s| s.parse().ok()).flatten(),
            take_profit: row.get::<Option<&str>, _>("take_profit").map(|s| s.parse().ok()).flatten(),
            exit_reason: row.get::<Option<&str>, _>("exit_reason").and_then(|s| match s {
                "signal" => Some(ExitType::Signal), "stop_loss" => Some(ExitType::StopLoss),
                "take_profit" => Some(ExitType::TakeProfit), "force_exit" => Some(ExitType::ForceExit),
                _ => None,
            }),
            profit_abs: row.get::<Option<&str>, _>("profit_abs").map(|s| s.parse().ok()).flatten(),
            profit_ratio: row.get::<Option<&str>, _>("profit_ratio").map(|s| s.parse().ok()).flatten(),
        })
    }
}

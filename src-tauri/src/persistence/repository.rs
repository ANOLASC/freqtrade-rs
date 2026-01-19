use crate::error::{AppError, Result};
use crate::types::*;
use chrono::{DateTime, Utc};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
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
        // Create parent directory if needed
        if let Some(parent) = db_path.as_ref().parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Config(format!("Failed to create database directory: {}", e)))?;
        }

        // Get absolute path for database
        let current_dir = std::env::current_dir().expect("Cannot get current directory");
        let abs_path = current_dir.join(db_path.as_ref());

        // Use absolute path format for SQLite with forward slashes
        let db_path_display = abs_path.to_string_lossy().replace("\\", "/");
        let db_url = format!("sqlite:{}?mode=rwc", db_path_display);
        eprintln!("Connecting to database: {}", db_url);

        let pool = SqlitePool::connect(&db_url)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Self::run_migrations(&pool).await?;
        Ok(Self { pool: Arc::new(pool) })
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        let migration_sql = include_str!("../../../migrations/001_initial.sql");
        for statement in migration_sql.split(";").map(|s| s.trim()).filter(|s| !s.is_empty()) {
            sqlx::query(statement)
                .execute(pool)
                .await
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
            .fetch_all(&*self.pool)
            .await?;
        rows.iter().map(|row| self.row_to_trade(row)).collect()
    }

    pub async fn get_all_trades(&self) -> Result<Vec<Trade>> {
        let rows = sqlx::query("SELECT * FROM trades ORDER BY open_date DESC")
            .fetch_all(&*self.pool)
            .await?;
        rows.iter().map(|row| self.row_to_trade(row)).collect()
    }

    pub async fn get_dashboard_stats(&self) -> Result<DashboardStats> {
        let stats_row = sqlx::query(
            r#"
            SELECT
                COUNT(*) as total_trades,
                COALESCE(SUM(CASE WHEN CAST(profit_ratio AS REAL) > 0 THEN 1 ELSE 0 END), 0) as winning_trades,
                COALESCE(SUM(is_open), 0) as open_trades,
                COALESCE(SUM(CAST(profit_abs AS REAL)), 0.0) as total_profit
            FROM trades
            "#,
        )
        .fetch_one(&*self.pool)
        .await?;

        let total_trades: i64 = stats_row.get("total_trades");
        let winning_trades: i64 = stats_row.get("winning_trades");
        let open_trades: i64 = stats_row.get("open_trades");
        let total_profit: f64 = stats_row.get("total_profit");

        let win_rate = if total_trades > 0 {
            (winning_trades as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        // Calculate max drawdown and total balance
        // We only fetch profit_abs to save bandwidth and allocations
        let rows = sqlx::query("SELECT CAST(profit_abs AS REAL) as profit_abs FROM trades ORDER BY open_date DESC")
            .fetch_all(&*self.pool)
            .await?;

        // Same logic as in commands.rs
        let mut peak_balance = Decimal::from(10000_i64);
        let mut current_balance = Decimal::from(10000_i64);
        let mut max_drawdown = 0.0;

        for row in rows {
            // If profit_abs is NULL, we skip or treat as 0? The original code:
            // if let Some(profit) = trade.profit_abs { current_balance += profit; }
            let profit_opt: Option<f64> = row.get("profit_abs");
            if let Some(profit_f64) = profit_opt {
                if let Some(profit) = Decimal::from_f64(profit_f64) {
                     current_balance += profit;
                }
            }

            if current_balance > peak_balance {
                peak_balance = current_balance;
            }
            let drawdown = (peak_balance - current_balance) / peak_balance * Decimal::from(100_i64);
             if drawdown > Decimal::try_from(max_drawdown).unwrap_or(Decimal::ZERO) {
                max_drawdown = drawdown.to_f64().unwrap_or(0.0);
            }
        }

        Ok(DashboardStats {
            total_profit,
            win_rate,
            open_trades: open_trades as usize,
            max_drawdown,
            total_balance: current_balance.to_f64().unwrap_or(0.0),
        })
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
            .bind(pair)
            .bind(timeframe)
            .bind(limit as i64)
            .fetch_all(&*self.pool)
            .await?;
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
            .fetch_all(&*self.pool)
            .await?;
        rows.iter()
            .map(|row| {
                let config_json: String = row.get("config");
                let result: BacktestResult = serde_json::from_str(&config_json).map_err(AppError::Serialization)?;
                Ok(result)
            })
            .collect()
    }

    fn get_decimal(&self, row: &sqlx::sqlite::SqliteRow, col: &str) -> Result<Decimal> {
        if let Ok(s) = row.try_get::<&str, _>(col) {
            s.parse()
                .map_err(|e| AppError::Parse(format!("Invalid decimal in {}: {}", col, e)))
        } else {
            let f: f64 = row
                .try_get(col)
                .map_err(|e| AppError::Database(format!("Failed to read {}: {}", col, e)))?;
            Decimal::from_f64(f).ok_or_else(|| {
                AppError::Parse(format!("Invalid float for decimal in {}", col))
            })
        }
    }

    fn get_opt_decimal(&self, row: &sqlx::sqlite::SqliteRow, col: &str) -> Result<Option<Decimal>> {
        if let Ok(Some(s)) = row.try_get::<Option<&str>, _>(col) {
            Ok(Some(s.parse().map_err(|e| {
                AppError::Parse(format!("Invalid decimal in {}: {}", col, e))
            })?))
        } else if let Ok(Some(f)) = row.try_get::<Option<f64>, _>(col) {
            Ok(Some(Decimal::from_f64(f).ok_or_else(|| {
                AppError::Parse(format!("Invalid float for decimal in {}", col))
            })?))
        } else {
            Ok(None)
        }
    }

    fn row_to_trade(&self, row: &sqlx::sqlite::SqliteRow) -> Result<Trade> {
        Ok(Trade {
            id: Uuid::parse_str(row.get("id"))
                .map_err(|e| AppError::Parse(format!("Invalid UUID: {}", e)))?,
            pair: row.get("pair"),
            is_open: row.get::<i32, _>("is_open") != 0,
            exchange: row.get("exchange"),
            open_rate: self.get_decimal(row, "open_rate")?,
            open_date: DateTime::parse_from_rfc3339(row.get("open_date"))
                .map_err(|e| AppError::Parse(format!("Invalid datetime: {}", e)))?
                .with_timezone(&Utc),
            close_rate: self.get_opt_decimal(row, "close_rate")?,
            close_date: row
                .get::<Option<&str>, _>("close_date")
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            amount: self.get_decimal(row, "amount")?,
            stake_amount: self.get_decimal(row, "stake_amount")?,
            strategy: row.get("strategy"),
            timeframe: match row.get::<&str, _>("timeframe") {
                "1m" => Timeframe::OneMinute,
                "5m" => Timeframe::FiveMinutes,
                "15m" => Timeframe::FifteenMinutes,
                "30m" => Timeframe::ThirtyMinutes,
                "1h" => Timeframe::OneHour,
                "4h" => Timeframe::FourHours,
                "1d" => Timeframe::OneDay,
                _ => Timeframe::OneHour,
            },
            stop_loss: self.get_opt_decimal(row, "stop_loss")?,
            take_profit: self.get_opt_decimal(row, "take_profit")?,
            exit_reason: row.get::<Option<&str>, _>("exit_reason").and_then(|s| match s {
                "signal" => Some(ExitType::Signal),
                "stop_loss" => Some(ExitType::StopLoss),
                "take_profit" => Some(ExitType::TakeProfit),
                "force_exit" => Some(ExitType::ForceExit),
                "stop_loss_on_exchange" => Some(ExitType::StopLossOnExchange),
                "emergency_exit" => Some(ExitType::EmergencyExit),
                "custom" => Some(ExitType::Custom),
                _ => None,
            }),
            profit_abs: self.get_opt_decimal(row, "profit_abs")?,
            profit_ratio: self.get_opt_decimal(row, "profit_ratio")?,
        })
    }

    fn row_to_kline(&self, row: &sqlx::sqlite::SqliteRow) -> Result<OHLCV> {
        Ok(OHLCV {
            timestamp: DateTime::parse_from_rfc3339(row.get("open_time"))
                .map_err(|e| AppError::Parse(format!("Invalid datetime: {}", e)))?
                .with_timezone(&Utc),
            open: self.get_decimal(row, "open")?,
            high: self.get_decimal(row, "high")?,
            low: self.get_decimal(row, "low")?,
            close: self.get_decimal(row, "close")?,
            volume: self.get_decimal(row, "volume")?,
        })
    }
}

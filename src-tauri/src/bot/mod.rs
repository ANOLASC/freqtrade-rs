use crate::error::Result;
use crate::types::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct TradingBot {
    status: Arc<tokio::sync::RwLock<BotStatus>>,
    exchange: Arc<dyn crate::exchange::Exchange>,
    strategy: Arc<dyn crate::strategy::Strategy>,
    repository: Arc<crate::persistence::Repository>,
    risk_manager: Option<Arc<crate::risk::RiskManager>>,
    config: crate::config::BotConfig,
}

impl TradingBot {
    pub fn new(
        exchange: Arc<dyn crate::exchange::Exchange>,
        strategy: Arc<dyn crate::strategy::Strategy>,
        repository: Arc<crate::persistence::Repository>,
        risk_manager: Option<Arc<crate::risk::RiskManager>>,
        config: crate::config::BotConfig,
    ) -> Self {
        Self {
            status: Arc::new(tokio::sync::RwLock::new(BotStatus::Stopped)),
            exchange,
            strategy,
            repository,
            risk_manager,
            config,
        }
    }

    pub async fn start(&self) -> Result<()> {
        *self.status.write().await = BotStatus::Running;

        let default_pair = "BTCUSDT";
        let default_timeframe = "1h";

        loop {
            let status = *self.status.read().await;

            match status {
                BotStatus::Running => {
                    // 检查全局停止
                    if let Some(risk_mgr) = &self.risk_manager
                        && let Some(stop_reason) = risk_mgr.check_global_stop().await?
                    {
                        eprintln!("Global stop triggered: {}", stop_reason.reason);
                        eprintln!("Unlock at: {}", stop_reason.until);
                        *self.status.write().await = BotStatus::Stopped;
                        break;
                    }

                    if let Err(e) = self.process_cycle(default_pair, default_timeframe).await {
                        eprintln!("Error processing cycle: {}", e);
                        *self.status.write().await = BotStatus::Error;
                        break;
                    }
                }
                BotStatus::Stopped => break,
                BotStatus::Paused => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
                BotStatus::Error => break,
            }

            let sleep_duration = if self.config.process_only_new_candles {
                tokio::time::Duration::from_secs(60)
            } else {
                tokio::time::Duration::from_secs(1)
            };

            tokio::time::sleep(sleep_duration).await;
        }

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        *self.status.write().await = BotStatus::Stopped;
        Ok(())
    }

    pub async fn pause(&self) -> Result<()> {
        *self.status.write().await = BotStatus::Paused;
        Ok(())
    }

    pub async fn get_status(&self) -> BotStatus {
        *self.status.read().await
    }

    async fn process_cycle(&self, pair: &str, timeframe: &str) -> Result<()> {
        // 获取 K线数据
        let klines = self.exchange.fetch_ohlcv(pair, timeframe, 500).await?;

        // 获取当前开仓交易
        let open_trades: Vec<Trade> = self.repository.get_open_trades().await?;

        // 处理卖信号
        let sell_signals = self.strategy.populate_sell_trend(&klines).await?;
        if !sell_signals.is_empty() && !open_trades.is_empty() {
            eprintln!("Got {} sell signals", sell_signals.len());
            // 执行卖出逻辑（简化版，实际实现需要根据信号确定卖哪对）
            if let Some(risk_mgr) = &self.risk_manager
                && let Some(stop_reason) = risk_mgr.check_pair_stop(pair).await?
            {
                eprintln!("Pair stop triggered for {}: {}", pair, stop_reason.reason);
                return Ok(());
            }
            // 执行卖出（dry_run 模式下只记录）
            if self.config.dry_run {
                eprintln!("[DRY RUN] Would sell {}", pair);
            } else {
                // TODO: 实现实际的卖出逻辑
                eprintln!("Sell signal for {}", pair);
            }
        }

        // 处理买信号
        if open_trades.len() < self.config.max_open_trades {
            let buy_signals = self.strategy.populate_buy_trend(&klines).await?;
            if !buy_signals.is_empty() {
                eprintln!("Got {} buy signals", buy_signals.len());
                // 执行买入（简化版，实际实现需要根据信号确定买哪对）
                if let Some(risk_mgr) = &self.risk_manager
                    && let Some(stop_reason) = risk_mgr.check_pair_stop(pair).await?
                {
                    eprintln!("Pair stop triggered for {}: {}", pair, stop_reason.reason);
                    return Ok(());
                }
                // 执行买入（dry_run 模式下只记录）
                if self.config.dry_run {
                    eprintln!("[DRY RUN] Would buy {}", pair);
                } else {
                    // TODO: 实现实际的买入逻辑
                    eprintln!("Buy signal for {}", pair);
                }
            }
        }

        Ok(())
    }
}

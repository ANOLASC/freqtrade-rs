use crate::error::Result;
use crate::types::*;
use chrono::Utc;
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

    #[allow(dead_code)]
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
        if !sell_signals.is_empty() {
            // Find trade for current pair
            if let Some(trade) = open_trades.iter().find(|t| t.pair == pair) {
                eprintln!("Got {} sell signals for {}", sell_signals.len(), pair);
                // 执行卖出逻辑
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
                    eprintln!("Sell signal for {}", pair);
                    let order_req = OrderRequest {
                        symbol: pair.to_string(),
                        side: TradeSide::Sell,
                        order_type: OrderType::Market,
                        amount: trade.amount,
                        price: None,
                    };

                    match self.exchange.create_order(order_req).await {
                        Ok(order) => {
                            eprintln!("Sell order executed: {:?}", order);
                            // Update Trade
                            // Use order price if available, otherwise fallback to current candle close
                            let close_price = order.price.unwrap_or(klines.last().unwrap().close);

                            let mut updated_trade = trade.clone();
                            updated_trade.is_open = false;
                            updated_trade.close_rate = Some(close_price);
                            updated_trade.close_date = Some(Utc::now());
                            updated_trade.exit_reason = Some(ExitType::Signal);

                            // Calculate profit
                            let profit_abs = (close_price - trade.open_rate) * trade.amount;
                            let profit_ratio = (close_price - trade.open_rate) / trade.open_rate;

                            updated_trade.profit_abs = Some(profit_abs);
                            updated_trade.profit_ratio = Some(profit_ratio);

                            if let Err(e) = self.repository.update_trade(&updated_trade).await {
                                eprintln!("Failed to update trade in DB: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute sell order: {}", e);
                        }
                    }
                }
            } else {
                eprintln!("Sell signals found for {} but no open trade exists", pair);
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

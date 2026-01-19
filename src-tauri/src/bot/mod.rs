use crate::error::Result;
use crate::types::*;
use std::sync::Arc;
use uuid::Uuid;

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
                    // Actual buy execution
                    // 1. Get current price (use last kline close as proxy for estimation)
                    let current_price = klines
                        .last()
                        .map(|k| k.close)
                        .unwrap_or(rust_decimal::Decimal::ZERO);

                    if current_price <= rust_decimal::Decimal::ZERO {
                        eprintln!("Invalid price for {}: {}", pair, current_price);
                        return Ok(());
                    }

                    // 2. Calculate amount based on stake amount
                    let stake_amount = rust_decimal::Decimal::from_f64_retain(self.config.stake_amount)
                        .unwrap_or(rust_decimal::Decimal::ZERO);

                    if stake_amount <= rust_decimal::Decimal::ZERO {
                        eprintln!("Invalid stake amount: {}", stake_amount);
                        return Ok(());
                    }

                    let amount = stake_amount / current_price;

                    // 3. Check balance
                    // Ensure we have enough of the stake currency (e.g. USDT) to cover the trade
                    let has_funds = match self.exchange.fetch_balance(&self.config.stake_currency).await {
                        Ok(balance) => {
                            if balance.free >= stake_amount {
                                true
                            } else {
                                eprintln!(
                                    "Insufficient funds for {}: required {}, available {}",
                                    pair, stake_amount, balance.free
                                );
                                false
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to check balance: {}", e);
                            // If check fails, we might want to skip or proceed depending on risk policy.
                            // Here we choose to skip to be safe.
                            false
                        }
                    };

                    if !has_funds {
                        return Ok(());
                    }

                    // 4. Create Order
                    let order_req = OrderRequest {
                        symbol: pair.to_string(),
                        side: TradeSide::Buy,
                        order_type: OrderType::Market,
                        amount: amount.round_dp(8), // Round to avoid precision issues
                        price: None,
                    };

                    match self.exchange.create_order(order_req).await {
                        Ok(order) => {
                            eprintln!("Buy order executed for {}: {:?}", pair, order);
                            // 5. Save trade to DB
                            let trade = Trade {
                                id: Uuid::new_v4(),
                                pair: pair.to_string(),
                                is_open: true,
                                exchange: self.exchange.get_name().to_string(),
                                open_rate: order.price.unwrap_or(current_price), // Use execution price or estimate
                                open_date: order.created_at,
                                close_rate: None,
                                close_date: None,
                                amount: order.amount,
                                stake_amount,
                                strategy: self.strategy.name().to_string(),
                                timeframe: match timeframe {
                                    "1m" => Timeframe::OneMinute,
                                    "5m" => Timeframe::FiveMinutes,
                                    "15m" => Timeframe::FifteenMinutes,
                                    "30m" => Timeframe::ThirtyMinutes,
                                    "1h" => Timeframe::OneHour,
                                    "4h" => Timeframe::FourHours,
                                    "1d" => Timeframe::OneDay,
                                    _ => Timeframe::OneHour,
                                },
                                stop_loss: None,   // Should be set by strategy/risk manager
                                take_profit: None, // Should be set by strategy/risk manager
                                exit_reason: None,
                                profit_abs: None,
                                profit_ratio: None,
                            };
                            if let Err(e) = self.repository.create_trade(&trade).await {
                                eprintln!("Failed to save trade for {}: {}", pair, e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute buy order for {}: {}", pair, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

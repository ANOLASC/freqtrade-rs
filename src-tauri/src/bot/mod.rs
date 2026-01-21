use crate::error::Result;
use crate::types::*;
use chrono::Utc;
use futures::future::try_join_all;
use rust_decimal::Decimal;
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

                    if let Err(e) = self.process_all_pairs().await {
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

    // New method to process all configured pairs
    #[cfg_attr(test, visibility::make(pub))]
    async fn process_all_pairs(&self) -> Result<()> {
        let pairs = self.config.trading_pairs.clone();

        if pairs.is_empty() {
            return Err(crate::error::AppError::Config(
                "No trading_pairs configured".to_string(),
            ));
        }

        let futures = pairs.iter().map(|pair| {
            let pair = pair.trim();
            async move {
                if pair.is_empty() || pair.contains(char::is_whitespace) {
                    return Err(crate::error::AppError::Config(format!(
                        "Invalid trading pair: {:?}",
                        pair
                    )));
                }
                match self.process_cycle(pair, &self.config.timeframe).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        eprintln!("Error processing pair {}: {}", pair, e);
                        // Continue processing other pairs even if one fails
                        Ok(())
                    }
                }
            }
        });

        try_join_all(futures).await?;

        Ok(())
    }

    #[cfg_attr(test, visibility::make(pub))]
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
                            // Use order price if available, otherwise fallback to current candle close
                            let close_price = match order.price {
                                Some(p) => p,
                                None => {
                                    klines
                                        .last()
                                        .ok_or_else(|| {
                                            crate::error::AppError::InvalidInput("No klines data available".to_string())
                                        })?
                                        .close
                                }
                            };

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
                eprintln!("Got {} buy signals for {}", buy_signals.len(), pair);

                // 检查交易对是否已有持仓
                if open_trades.iter().any(|t| t.pair == pair) {
                    eprintln!("Already have open position for {}", pair);
                    return Ok(());
                }

                // 检查风控
                if let Some(risk_mgr) = &self.risk_manager
                    && let Some(stop_reason) = risk_mgr.check_pair_stop(pair).await?
                {
                    eprintln!("Pair stop triggered for {}: {}", pair, stop_reason.reason);
                    return Ok(());
                }

                // 计算买入金额和数量
                let current_price = klines
                    .last()
                    .ok_or_else(|| crate::error::AppError::InvalidInput("No klines data available".to_string()))?
                    .close;
                let stake_amount_f64 = self.config.stake_amount;
                let stake_amount_decimal = Decimal::try_from(stake_amount_f64).unwrap_or(Decimal::from(0));

                let amount = if stake_amount_decimal > Decimal::ZERO {
                    stake_amount_decimal / current_price
                } else {
                    return Err(crate::error::AppError::InvalidInput("Invalid stake amount".to_string()));
                };

                // dry_run 模式
                if self.config.dry_run {
                    eprintln!(
                        "[DRY RUN] Would buy {} - Amount: {} @ Price: {}",
                        pair, amount, current_price
                    );

                    // 创建模拟交易记录
                    let trade = Trade {
                        id: Uuid::new_v4(),
                        pair: pair.to_string(),
                        is_open: true,
                        exchange: self.exchange.get_name().to_string(),
                        open_rate: current_price,
                        open_date: Utc::now(),
                        close_rate: None,
                        close_date: None,
                        amount,
                        stake_amount: stake_amount_decimal,
                        strategy: self.strategy.name().to_string(),
                        timeframe: Timeframe::OneHour,
                        stop_loss: None,
                        take_profit: None,
                        exit_reason: None,
                        profit_abs: None,
                        profit_ratio: None,
                    };

                    if let Err(e) = self.repository.create_trade(&trade).await {
                        eprintln!("Failed to create trade in DB: {}", e);
                    }

                    return Ok(());
                }

                // 实盘模式 - 执行买入
                eprintln!(
                    "Executing buy for {} - Amount: {} @ Price: {}",
                    pair, amount, current_price
                );
                let order_req = OrderRequest {
                    symbol: pair.to_string(),
                    side: TradeSide::Buy,
                    order_type: OrderType::Market,
                    amount,
                    price: None,
                };

                match self.exchange.create_order(order_req).await {
                    Ok(order) => {
                        eprintln!("Buy order executed: {:?}", order);

                        // 计算实际成交价格 (避免除零)
                        let avg_price = order.price.unwrap_or_else(|| {
                            if order.amount > Decimal::ZERO {
                                order.filled * current_price / order.amount
                            } else {
                                current_price
                            }
                        });

                        // 创建交易记录
                        let trade = Trade {
                            id: Uuid::new_v4(),
                            pair: pair.to_string(),
                            is_open: true,
                            exchange: self.exchange.get_name().to_string(),
                            open_rate: avg_price,
                            open_date: Utc::now(),
                            close_rate: None,
                            close_date: None,
                            amount: order.filled,
                            stake_amount: stake_amount_decimal,
                            strategy: self.strategy.name().to_string(),
                            timeframe: Timeframe::OneHour,
                            stop_loss: None,
                            take_profit: None,
                            exit_reason: None,
                            profit_abs: None,
                            profit_ratio: None,
                        };

                        if let Err(e) = self.repository.create_trade(&trade).await {
                            eprintln!("Failed to create trade in DB: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute buy order: {}", e);
                        return Err(crate::error::AppError::Exchange(format!("Buy order failed: {}", e)));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;

// Persistence层测试模块
// 迁移自 freqtrade Python项目的 tests/persistence/test_persistence.py
// 原文件: 2,895行, 包含150+个测试用例

#[cfg(test)]
pub mod trade_tests {
    use approx::assert_relative_eq;
    use chrono::{DateTime, Duration, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::exchange::Exchange;
    use crate::persistence::{
        models::TradingMode,
        order::{Order, OrderSide, OrderStatus, OrderType},
        repository::Repository,
        trade::Trade,
    };
    use crate::types::{ProfitCalculation, TradeDirection};

    // ============ 基础测试夹具 ============

    /// 模拟Python的fee fixture
    fn mock_fee() -> Decimal {
        Decimal::from_str("0.0025").unwrap()
    }

    /// 创建标准测试交易 (对应Python的 conftest_trades.py)
    async fn create_test_trade(pair: &str, is_short: bool, leverage: u32, trading_mode: TradingMode) -> Trade {
        Trade::builder()
            .id(2)
            .pair(pair.to_string())
            .stake_amount(Decimal::from(60))
            .open_rate(Decimal::from_str("2.0").unwrap())
            .amount(Decimal::from(30))
            .is_open(true)
            .open_date(Utc::now() - Duration::minutes(10))
            .fee_open(mock_fee())
            .fee_close(mock_fee())
            .exchange("binance".to_string())
            .is_short(is_short)
            .leverage(Decimal::from(leverage))
            .trading_mode(trading_mode)
            .build()
            .await
    }

    // ============ Line 26-47: test_enter_exit_side ============

    /// 测试交易的enter/exit方向
    /// 原Python测试: test_persistence.py:26-47
    #[tokio::test]
    async fn test_trade_entry_exit_side() {
        let test_cases = vec![(false, "buy", "sell", "long"), (true, "sell", "buy", "short")];

        for (is_short, expected_entry, expected_exit, expected_direction) in test_cases {
            let trade = create_test_trade("ADA/USDT", is_short, 2, TradingMode::Margin).await;

            assert_eq!(
                trade.entry_side().as_str(),
                expected_entry,
                "Entry side mismatch for is_short={}",
                is_short
            );
            assert_eq!(
                trade.exit_side().as_str(),
                expected_exit,
                "Exit side mismatch for is_short={}",
                is_short
            );
            assert_eq!(
                trade.trade_direction().as_str(),
                expected_direction,
                "Trade direction mismatch for is_short={}",
                is_short
            );
        }
    }

    // ============ Line 51-176: test_set_stop_loss_liquidation ============

    /// 测试止损位和清算价格设置
    /// 原Python测试: test_persistence.py:51-176
    #[tokio::test]
    async fn test_trade_set_stop_loss_liquidation() {
        let trade = create_test_trade("ADA/USDT", false, 2, TradingMode::Margin).await;

        // 设置清算价格
        trade.set_liquidation_price(Decimal::from_str("0.09").unwrap()).await;
        assert_eq!(trade.liquidation_price, Some(Decimal::from_str("0.09").unwrap()));
        assert!(trade.stop_loss.is_none());
        assert!(trade.initial_stop_loss.is_none());

        // 调整止损位 (allow_refresh=true)
        trade
            .adjust_stop_loss(Decimal::from_str("2.0").unwrap(), Decimal::from(-0.2), true)
            .await;

        assert_eq!(trade.liquidation_price, Some(Decimal::from_str("0.09").unwrap()));
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.8").unwrap()));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.8").unwrap()));

        // 再次设置清算价格
        trade.set_liquidation_price(Decimal::from_str("0.08").unwrap()).await;
        assert_eq!(trade.liquidation_price, Some(Decimal::from_str("0.08").unwrap()));
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.8").unwrap()));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.8").unwrap()));

        // 设置更高的清算价格
        trade.set_liquidation_price(Decimal::from_str("0.11").unwrap()).await;
        trade
            .adjust_stop_loss(Decimal::from_str("2.0").unwrap(), Decimal::from(-0.1), false)
            .await;

        assert_eq!(trade.liquidation_price, Some(Decimal::from_str("0.11").unwrap()));
        // 止损位不随清算价格变化
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.8").unwrap()));
        assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.2)));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.8").unwrap()));

        // 当前价格更低，不应该移动止损
        trade
            .adjust_stop_loss(Decimal::from_str("1.8").unwrap(), Decimal::from(-0.1), false)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.8").unwrap()));

        // 当前价格更低但allow_refresh=true，应该移动
        trade
            .adjust_stop_loss(Decimal::from_str("1.8").unwrap(), Decimal::from(-0.22), true)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.8").unwrap())); // 保持不变
        assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.22)));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.8").unwrap()));

        // 当前价格更高，应该移动止损
        trade
            .adjust_stop_loss(Decimal::from_str("2.1").unwrap(), Decimal::from(-0.1), false)
            .await;
        assert_relative_eq!(
            trade.stop_loss.unwrap(),
            Decimal::from_str("1.994999").unwrap(),
            max_relative = 0.0001
        );
        assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.1)));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.8").unwrap()));
    }

    // ============ Line 177-279: test_interest ============

    /// 测试利息计算 (参数化测试)
    /// 原Python测试: test_persistence.py:177-279
    #[tokio::test]
    async fn test_trade_interest() {
        struct TestCase {
            exchange: String,
            is_short: bool,
            lev: u32,
            minutes: u32,
            rate: Decimal,
            expected_interest: Decimal,
            trading_mode: TradingMode,
        }

        let test_cases = vec![
            // Binance 3x leverage, 10 minutes
            TestCase {
                exchange: "binance".to_string(),
                is_short: false,
                lev: 3,
                minutes: 10,
                rate: Decimal::from_str("0.0005").unwrap(),
                expected_interest: Decimal::from_str("0.00083333").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            // Binance short 3x, 10 minutes
            TestCase {
                exchange: "binance".to_string(),
                is_short: true,
                lev: 3,
                minutes: 10,
                rate: Decimal::from_str("0.0005").unwrap(),
                expected_interest: Decimal::from_str("0.000625").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            // Kraken 3x leverage, 10 minutes
            TestCase {
                exchange: "kraken".to_string(),
                is_short: false,
                lev: 3,
                minutes: 10,
                rate: Decimal::from_str("0.0005").unwrap(),
                expected_interest: Decimal::from_str("0.04").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            // Kraken short 3x, 10 minutes
            TestCase {
                exchange: "kraken".to_string(),
                is_short: true,
                lev: 3,
                minutes: 10,
                rate: Decimal::from_str("0.0005").unwrap(),
                expected_interest: Decimal::from_str("0.03").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            // Binance 5x leverage, 295 minutes
            TestCase {
                exchange: "binance".to_string(),
                is_short: false,
                lev: 5,
                minutes: 295,
                rate: Decimal::from_str("0.0005").unwrap(),
                expected_interest: Decimal::from_str("0.005").unwrap(),
                trading_mode: TradingMode::Margin,
            },
        ];

        for tc in test_cases {
            let exchange = if tc.exchange == "binance" {
                Exchange::Binance
            } else {
                Exchange::Kraken
            };

            let trade = Trade::builder()
                .pair("ADA/USDT".to_string())
                .stake_amount(Decimal::from(20))
                .amount(Decimal::from(30))
                .open_rate(Decimal::from_str("2.0").unwrap())
                .open_date(Utc::now() - Duration::minutes(tc.minutes))
                .fee_open(tc.rate)
                .fee_close(tc.rate)
                .exchange(tc.exchange.clone())
                .leverage(Decimal::from(tc.lev))
                .interest_rate(tc.rate)
                .is_short(tc.is_short)
                .trading_mode(tc.trading_mode)
                .build()
                .await;

            let calculated_interest = trade.calculate_interest(tc.minutes).await;

            assert_relative_eq!(
                calculated_interest,
                tc.expected_interest,
                max_relative = 0.01,
                "{} interest mismatch for {} short={} lev={} min={}",
                tc.exchange,
                tc.exchange,
                tc.is_short,
                tc.lev,
                tc.minutes
            );
        }
    }

    // ============ Line 281-371: test_borrowed ============

    /// 测试借款金额计算
    /// 原Python测试: test_persistence.py:281-371
    #[tokio::test]
    async fn test_trade_borrowed() {
        struct BorrowTestCase {
            is_short: bool,
            lev: f64,
            borrowed: f64,
            trading_mode: TradingMode,
        }

        let test_cases = vec![
            BorrowTestCase {
                is_short: false,
                lev: 1.0,
                borrowed: 0.0,
                trading_mode: TradingMode::Spot,
            },
            BorrowTestCase {
                is_short: true,
                lev: 1.0,
                borrowed: 30.0,
                trading_mode: TradingMode::Margin,
            },
            BorrowTestCase {
                is_short: false,
                lev: 3.0,
                borrowed: 40.0,
                trading_mode: TradingMode::Margin,
            },
            BorrowTestCase {
                is_short: true,
                lev: 3.0,
                borrowed: 30.0,
                trading_mode: TradingMode::Margin,
            },
        ];

        for tc in test_cases {
            let trade = Trade::builder()
                .id(2)
                .pair("ADA/USDT".to_string())
                .stake_amount(Decimal::from(60))
                .open_rate(Decimal::from_str("2.0").unwrap())
                .amount(Decimal::from(30))
                .is_open(true)
                .open_date(Utc::now())
                .fee_open(mock_fee())
                .fee_close(mock_fee())
                .exchange("binance".to_string())
                .is_short(tc.is_short)
                .leverage(Decimal::from_f64(tc.lev).unwrap())
                .trading_mode(tc.trading_mode)
                .build()
                .await;

            assert_eq!(
                trade.borrowed,
                Decimal::from_f64(tc.borrowed).unwrap(),
                "Borrowed mismatch for is_short={} lev={}",
                tc.is_short,
                tc.lev
            );
        }
    }

    // ============ Line 373-520: test_update_limit_order ============

    /// 测试限价单更新
    /// 原Python测试: test_persistence.py:373-520
    #[tokio::test]
    async fn test_trade_update_limit_order() {
        // 测试用例: (is_short, open_rate, close_rate, lev, profit, trading_mode)
        struct LimitOrderTestCase {
            is_short: bool,
            open_rate: Decimal,
            close_rate: Decimal,
            lev: Decimal,
            profit: Decimal,
            trading_mode: TradingMode,
        }

        let test_cases = vec![
            LimitOrderTestCase {
                is_short: false,
                open_rate: Decimal::from_str("2.0").unwrap(),
                close_rate: Decimal::from_str("2.2").unwrap(),
                lev: Decimal::from(1),
                profit: Decimal::from_str("0.09451372").unwrap(),
                trading_mode: TradingMode::Spot,
            },
            LimitOrderTestCase {
                is_short: true,
                open_rate: Decimal::from_str("2.2").unwrap(),
                close_rate: Decimal::from_str("2.0").unwrap(),
                lev: Decimal::from(3),
                profit: Decimal::from_str("0.25894253").unwrap(),
                trading_mode: TradingMode::Margin,
            },
        ];

        for tc in test_cases {
            let trade = Trade::builder()
                .id(2)
                .pair("ADA/USDT".to_string())
                .stake_amount(Decimal::from(60))
                .open_rate(tc.open_rate)
                .amount(Decimal::from(30))
                .is_open(true)
                .open_date(Utc::now() - Duration::minutes(10))
                .fee_open(mock_fee())
                .fee_close(mock_fee())
                .exchange("binance".to_string())
                .is_short(tc.is_short)
                .interest_rate(Decimal::from_str("0.0005").unwrap())
                .leverage(tc.lev)
                .trading_mode(tc.trading_mode)
                .build()
                .await;

            assert!(!trade.has_open_orders());
            assert!(trade.close_profit.is_none());
            assert!(trade.close_date.is_none());

            // 模拟入场订单
            let enter_order = Order::from_ccxt_object(
                &create_limit_buy_order("ADA/USDT", tc.open_rate, Decimal::from(30), "buy"),
                "ADA/USDT",
                OrderSide::Buy,
            )
            .await;

            trade.orders.write().await.push(enter_order.clone());
            trade.update_trade(&enter_order).await;

            assert!(!trade.has_open_orders());
            assert_eq!(trade.open_rate, tc.open_rate);
            assert!(trade.close_profit.is_none());
            assert!(trade.close_date.is_none());

            // 模拟出场订单
            let exit_order = Order::from_ccxt_object(
                &create_limit_sell_order("ADA/USDT", tc.close_rate, Decimal::from(30), "sell"),
                "ADA/USDT",
                OrderSide::Sell,
            )
            .await;

            trade.orders.write().await.push(exit_order.clone());
            trade.update_trade(&exit_order).await;

            assert!(!trade.has_open_orders());
            assert_eq!(trade.close_rate, Some(tc.close_rate));
            assert_relative_eq!(trade.close_profit.unwrap(), tc.profit, max_relative = 0.0001);
            assert!(trade.close_date.is_some());
        }
    }

    // ============ Line 776-830: test_calc_open_trade_value ============

    /// 测试开仓价值计算
    /// 原Python测试: test_persistence.py:776-830
    #[tokio::test]
    async fn test_trade_calc_open_trade_value() {
        struct OpenValueTestCase {
            exchange: String,
            is_short: bool,
            lev: u32,
            fee_rate: Decimal,
            result: Decimal,
            trading_mode: TradingMode,
        }

        let test_cases = vec![
            // 0.25% fee cases
            OpenValueTestCase {
                exchange: "binance".to_string(),
                is_short: false,
                lev: 1,
                fee_rate: Decimal::from_str("0.0025").unwrap(),
                result: Decimal::from_str("60.15").unwrap(),
                trading_mode: TradingMode::Spot,
            },
            OpenValueTestCase {
                exchange: "binance".to_string(),
                is_short: false,
                lev: 3,
                fee_rate: Decimal::from_str("0.0025").unwrap(),
                result: Decimal::from_str("60.15").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            OpenValueTestCase {
                exchange: "binance".to_string(),
                is_short: true,
                lev: 1,
                fee_rate: Decimal::from_str("0.0025").unwrap(),
                result: Decimal::from_str("59.85").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            OpenValueTestCase {
                exchange: "binance".to_string(),
                is_short: true,
                lev: 3,
                fee_rate: Decimal::from_str("0.0025").unwrap(),
                result: Decimal::from_str("59.85").unwrap(),
                trading_mode: TradingMode::Margin,
            },
            // 0.3% fee cases
            OpenValueTestCase {
                exchange: "binance".to_string(),
                is_short: false,
                lev: 1,
                fee_rate: Decimal::from_str("0.003").unwrap(),
                result: Decimal::from_str("60.18").unwrap(),
                trading_mode: TradingMode::Spot,
            },
            OpenValueTestCase {
                exchange: "binance".to_string(),
                is_short: true,
                lev: 1,
                fee_rate: Decimal::from_str("0.003").unwrap(),
                result: Decimal::from_str("59.82").unwrap(),
                trading_mode: TradingMode::Margin,
            },
        ];

        for tc in test_cases {
            let trade = Trade::builder()
                .pair("ADA/USDT".to_string())
                .stake_amount(Decimal::from(60))
                .amount(Decimal::from(30))
                .open_rate(Decimal::from_str("2.0").unwrap())
                .open_date(Utc::now() - Duration::minutes(10))
                .fee_open(tc.fee_rate)
                .fee_close(tc.fee_rate)
                .exchange(tc.exchange.clone())
                .leverage(Decimal::from(tc.lev))
                .is_short(tc.is_short)
                .trading_mode(tc.trading_mode)
                .build()
                .await;

            let open_value = trade.calc_open_trade_value().await;

            assert_relative_eq!(
                open_value,
                tc.result,
                max_relative = 0.0001,
                "Open value mismatch for {} short={} lev={} fee={}%",
                tc.exchange,
                tc.is_short,
                tc.lev,
                tc.fee_rate * Decimal::from(100)
            );
        }
    }

    // ============ Line 1206-1256: test_adjust_stop_loss ============

    /// 测试止损调整逻辑
    /// 原Python测试: test_persistence.py:1206-1256
    #[tokio::test]
    async fn test_trade_adjust_stop_loss() {
        let trade = Trade::builder()
            .pair("ADA/USDT".to_string())
            .stake_amount(Decimal::from(30))
            .amount(Decimal::from(30))
            .open_rate(Decimal::from(1))
            .exchange("binance".to_string())
            .build()
            .await;

        // 初始调整 - 设置止损
        trade
            .adjust_stop_loss(Decimal::from(1), Decimal::from(-0.05), true)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("0.95").unwrap()));
        assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.05)));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("0.95").unwrap()));
        assert_eq!(trade.initial_stop_loss_pct, Some(Decimal::from(-0.05)));

        // 当前价格更高，但低于最高价，不应该移动止损
        trade
            .adjust_stop_loss(Decimal::from_str("0.96").unwrap(), Decimal::from(-0.05), false)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("0.95").unwrap()));

        // 当前价格更高，用更高比例调整，应该移动
        trade
            .adjust_stop_loss(Decimal::from_str("1.3").unwrap(), Decimal::from(-0.1), false)
            .await;
        assert_relative_eq!(
            trade.stop_loss.unwrap(),
            Decimal::from_str("1.17").unwrap(),
            max_relative = 0.0001
        );
        assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.1)));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("0.95").unwrap()));

        // 当前价格更高但不是新高，不应该移动
        trade
            .adjust_stop_loss(Decimal::from_str("1.2").unwrap(), Decimal::from(0.1), false)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.17").unwrap()));

        // 当前价格创短期新高，应该移动
        trade
            .adjust_stop_loss(Decimal::from_str("1.4").unwrap(), Decimal::from(0.1), false)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.26").unwrap()));
    }

    // ============ Line 1258-1300: test_adjust_stop_loss_short ============

    /// 测试做空交易止损调整
    /// 原Python测试: test_persistence.py:1258-1300
    #[tokio::test]
    async fn test_trade_adjust_stop_loss_short() {
        let trade = Trade::builder()
            .pair("ADA/USDT".to_string())
            .stake_amount(Decimal::from(1))
            .amount(Decimal::from(5))
            .open_rate(Decimal::from(1))
            .exchange("binance".to_string())
            .is_short(true)
            .build()
            .await;

        // 做空时，止损在开仓价上方
        trade
            .adjust_stop_loss(Decimal::from(1), Decimal::from(-0.05), true)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.05").unwrap()));
        assert_eq!(trade.stop_loss_pct, Some(Decimal::from(-0.05)));
        assert_eq!(trade.initial_stop_loss, Some(Decimal::from_str("1.05").unwrap()));

        // 价格小幅上涨但未突破，不移动止损
        trade
            .adjust_stop_loss(Decimal::from_str("1.04").unwrap(), Decimal::from(-0.05), false)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("1.05").unwrap()));

        // 价格下跌，应该移动止损（做空时价格越低，止损越低）
        trade
            .adjust_stop_loss(Decimal::from_str("0.7").unwrap(), Decimal::from(-0.1), false)
            .await;
        // 如果价格跌到0.7，10%追踪止损应该在0.7*1.1 = 0.77
        assert_relative_eq!(
            trade.stop_loss.unwrap(),
            Decimal::from_str("0.77").unwrap(),
            max_relative = 0.0001
        );

        // 价格小幅反弹，不移动
        trade
            .adjust_stop_loss(Decimal::from_str("0.8").unwrap(), Decimal::from(-0.1), false)
            .await;
        assert_eq!(trade.stop_loss, Some(Decimal::from_str("0.77").unwrap()));

        // 价格继续下跌，继续移动
        trade
            .adjust_stop_loss(Decimal::from_str("0.6").unwrap(), Decimal::from(-0.1), false)
            .await;
        assert_relative_eq!(
            trade.stop_loss.unwrap(),
            Decimal::from_str("0.66").unwrap(),
            max_relative = 0.0001
        );
    }

    // ============ 辅助函数 ============

    /// 创建模拟的限价买入订单
    fn create_limit_buy_order(symbol: &str, price: Decimal, amount: Decimal, side: &str) -> serde_json::Value {
        serde_json::json!({
            "id": "12345",
            "symbol": symbol,
            "status": "closed",
            "side": side,
            "type": "limit",
            "price": price.to_string(),
            "amount": amount.to_string(),
            "filled": amount.to_string(),
            "cost": (price * amount).to_string(),
            "remaining": "0",
            "average": price.to_string()
        })
    }

    /// 创建模拟的限价卖出订单
    fn create_limit_sell_order(symbol: &str, price: Decimal, amount: Decimal, side: &str) -> serde_json::Value {
        serde_json::json!({
            "id": "12346",
            "symbol": symbol,
            "status": "closed",
            "side": side,
            "type": "limit",
            "price": price.to_string(),
            "amount": amount.to_string(),
            "filled": amount.to_string(),
            "cost": (price * amount).to_string(),
            "remaining": "0",
            "average": price.to_string()
        })
    }
}

#[cfg(test)]
pub mod order_tests {
    // Order模型测试将在此模块中实现
}

#[cfg(test)]
pub mod repository_tests {
    // Repository测试将在此模块中实现
}

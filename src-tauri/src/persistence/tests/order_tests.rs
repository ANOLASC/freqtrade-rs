// Order type tests for freqtrade-rs
// Tests for Order-related types from types.rs

#[cfg(test)]
mod order_tests {
    use crate::types::{OrderStatus, OrderType, TradeSide};

    // ============ Order Type Tests ============

    #[test]
    fn test_order_type_variants() {
        let market_order = OrderType::Market;
        let limit_order = OrderType::Limit;
        let stop_limit = OrderType::StopLimit;
        let stop_market = OrderType::StopMarket;

        assert_eq!(format!("{:?}", market_order), "Market");
        assert_eq!(format!("{:?}", limit_order), "Limit");
        assert_eq!(format!("{:?}", stop_limit), "StopLimit");
        assert_eq!(format!("{:?}", stop_market), "StopMarket");
    }

    #[test]
    fn test_order_status_variants() {
        let statuses = [
            OrderStatus::New,
            OrderStatus::PartiallyFilled,
            OrderStatus::Filled,
            OrderStatus::Canceled,
            OrderStatus::Rejected,
            OrderStatus::Expired,
        ];

        for status in statuses {
            let debug_name = format!("{:?}", status);
            assert!(!debug_name.is_empty());
        }
    }

    #[test]
    fn test_trade_side_variants() {
        let buy = TradeSide::Buy;
        let sell = TradeSide::Sell;

        assert_eq!(format!("{:?}", buy), "Buy");
        assert_eq!(format!("{:?}", sell), "Sell");
    }

    #[test]
    fn test_order_serialization() {
        // Test that order types can be serialized to lowercase
        use serde::Serialize;

        #[derive(Serialize)]
        struct TestOrder {
            order_type: OrderType,
            status: OrderStatus,
            side: TradeSide,
        }

        let order = TestOrder {
            order_type: OrderType::Limit,
            status: OrderStatus::New,
            side: TradeSide::Buy,
        };

        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("\"limit\""));
        assert!(json.contains("\"new\""));
        assert!(json.contains("\"buy\""));
    }

    #[test]
    fn test_order_type_equality() {
        let order1 = OrderType::Limit;
        let order2 = OrderType::Limit;
        let order3 = OrderType::Market;

        assert_eq!(order1, order2);
        assert_ne!(order1, order3);
    }

    #[test]
    fn test_order_status_equality() {
        let status1 = OrderStatus::Filled;
        let status2 = OrderStatus::Filled;
        let status3 = OrderStatus::Canceled;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_trade_side_equality() {
        let side1 = TradeSide::Buy;
        let side2 = TradeSide::Buy;
        let side3 = TradeSide::Sell;

        assert_eq!(side1, side2);
        assert_ne!(side1, side3);
    }
}

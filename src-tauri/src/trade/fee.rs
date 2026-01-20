use rust_decimal::Decimal;

/// Fee structure for different order types and exchanges
#[derive(Debug, Clone, Copy)]
pub struct Fee {
    /// Maker fee rate (0.1% = 0.001)
    pub maker_fee: Decimal,
    /// Taker fee rate (0.1% = 0.001)
    pub taker_fee: Decimal,
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            maker_fee: Decimal::from(1) / Decimal::from(1000), // 0.1%
            taker_fee: Decimal::from(1) / Decimal::from(1000), // 0.1%
        }
    }
}

/// Calculate fee for a trade
///
/// # Arguments
/// * `amount` - Trade amount (in base currency, e.g., BTC)
/// * `price` - Execution price (in quote currency, e.g., USDT)
/// * `fee_rate` - Fee rate (e.g., 0.001 for 0.1%)
/// * `fee_currency` - Currency to pay fee in ("base" or "quote")
///
/// # Returns
/// Fee amount in the specified currency
pub fn calculate_fee(amount: Decimal, price: Decimal, fee_rate: Decimal, fee_currency: &str) -> Decimal {
    let total_value = amount * price;

    match fee_currency {
        "base" => amount * fee_rate, // Fee in base currency (e.g., BTC)
        _ => total_value * fee_rate, // Fee in quote currency (e.g., USDT)
    }
}

/// Calculate total cost including fees
///
/// Returns (total_cost, fee_amount)
pub fn calculate_total_cost(
    amount: Decimal,
    price: Decimal,
    fee_rate: Decimal,
    fee_currency: &str,
) -> (Decimal, Decimal) {
    let fee = calculate_fee(amount, price, fee_rate, fee_currency);
    let total = amount * price + fee;
    (total, fee)
}

/// Calculate slippage for an order
///
/// # Arguments
/// * `expected_price` - Expected execution price
/// * `actual_price` - Actual execution price
///
/// # Returns
/// Slippage as a decimal (e.g., 0.001 for 0.1% slippage)
pub fn calculate_slippage(expected_price: Decimal, actual_price: Decimal) -> Decimal {
    if expected_price.is_zero() {
        return Decimal::ZERO;
    }
    (actual_price - expected_price).abs() / expected_price
}

/// Simulate slippage for backtesting
///
/// # Arguments
/// * `base_price` - Current market price
/// * `_amount` - Order amount (reserved for future use)
/// * `order_type` - "market" or "limit"
/// * `slippage_ratio` - Expected slippage ratio (e.g., 0.001 for 0.1%)
///
/// # Returns
/// Adjusted price with slippage applied
pub fn simulate_slippage(base_price: Decimal, _amount: Decimal, order_type: &str, slippage_ratio: Decimal) -> Decimal {
    match order_type {
        "market" => {
            // Market orders typically have slippage
            // Use a fixed slippage within the ratio for simplicity
            let max_slippage = slippage_ratio * Decimal::from(2); // Market orders can have up to 2x slippage
            base_price * (Decimal::ONE + max_slippage)
        }
        "limit" => {
            // Limit orders should have minimal slippage (execution at specified price or better)
            base_price
        }
        _ => base_price,
    }
}

/// Calculate Binance trading fee based on VIP level and maker/taker status
///
/// # Arguments
/// * `is_maker` - Whether this is a maker order (adds liquidity)
/// * `vip_level` - Binance VIP level (0-9)
///
/// # Returns
/// Fee rate as decimal (e.g., 0.001 for 0.1%)
///
/// Fee structure based on Binance Spot trading (as of 2024):
/// - VIP 0: Maker 0.1%, Taker 0.1%
/// - VIP 1: Maker 0.09%, Taker 0.1%
/// - VIP 2: Maker 0.08%, Taker 0.1%
/// - VIP 3+: Lower fees (simplified to 0.06% maker, 0.08% taker)
pub fn binance_fee(is_maker: bool, vip_level: u8) -> Decimal {
    match vip_level {
        0 => Decimal::from(1) / Decimal::from(1000), // 0.1% for both
        1 => {
            if is_maker {
                Decimal::from(9) / Decimal::from(10000) // 0.09%
            } else {
                Decimal::from(1) / Decimal::from(1000) // 0.1%
            }
        }
        2 => {
            if is_maker {
                Decimal::from(8) / Decimal::from(10000) // 0.08%
            } else {
                Decimal::from(1) / Decimal::from(1000) // 0.1%
            }
        }
        _ => {
            // VIP 3+ simplified
            if is_maker {
                Decimal::from(6) / Decimal::from(10000) // 0.06%
            } else {
                Decimal::from(8) / Decimal::from(10000) // 0.08%
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::ToPrimitive;

    #[test]
    fn test_calculate_fee_base_currency() {
        let amount = Decimal::from(1); // 1 BTC
        let price = Decimal::from(50000); // $50,000
        let fee_rate = Decimal::from(1) / Decimal::from(1000); // 0.1%

        let fee = calculate_fee(amount, price, fee_rate, "base");

        // 1 BTC * 0.001 = 0.001 BTC
        assert_eq!(fee.to_f64().unwrap(), 0.001);
    }

    #[test]
    fn test_calculate_fee_quote_currency() {
        let amount = Decimal::from(1); // 1 BTC
        let price = Decimal::from(50000); // $50,000
        let fee_rate = Decimal::from(1) / Decimal::from(1000); // 0.1%

        let fee = calculate_fee(amount, price, fee_rate, "quote");

        // 50000 * 0.001 = $50
        assert_eq!(fee.to_f64().unwrap(), 50.0);
    }

    #[test]
    fn test_calculate_slippage() {
        let expected = Decimal::from(50000);
        let actual = Decimal::from(50250);

        let slippage = calculate_slippage(expected, actual);

        // |50250 - 50000| / 50000 = 0.005 = 0.5%
        assert_eq!(slippage.to_f64().unwrap(), 0.005);
    }

    #[test]
    fn test_zero_slippage() {
        let price = Decimal::from(50000);

        let slippage = calculate_slippage(price, price);

        assert_eq!(slippage, Decimal::ZERO);
    }

    #[test]
    fn test_total_cost() {
        let amount = Decimal::from(1); // 1 BTC
        let price = Decimal::from(50000); // $50,000
        let fee_rate = Decimal::from(1) / Decimal::from(1000); // 0.1%

        let (total, fee) = calculate_total_cost(amount, price, fee_rate, "quote");

        // Total: 50000 + 50 = 50050
        assert_eq!(total.to_f64().unwrap(), 50050.0);
        // Fee: 50
        assert_eq!(fee.to_f64().unwrap(), 50.0);
    }
}

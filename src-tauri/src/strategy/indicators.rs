use crate::error::Result;
use crate::types::OHLCV;
use rust_decimal::Decimal;

pub struct SMA {
    period: usize,
}

impl SMA {
    pub fn new(period: usize) -> Self {
        Self { period }
    }

    pub fn calculate(&self, data: &[OHLCV]) -> Result<Vec<Option<Decimal>>> {
        if data.len() < self.period {
            return Ok(vec![None; data.len()]);
        }

        let mut result = vec![None; self.period - 1];

        for i in self.period - 1..data.len() {
            let start_idx = i + 1 - self.period;
            let sum: Decimal = data[start_idx..=i].iter().map(|c| c.close).sum();
            result.push(Some(sum / Decimal::from(self.period as u64)));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use chrono::Utc;

    fn create_ohlcv(close_prices: Vec<&str>) -> Vec<OHLCV> {
        close_prices
            .into_iter()
            .enumerate()
            .map(|(_i, price)| OHLCV {
                timestamp: Utc::now(),
                open: Decimal::ZERO,
                high: Decimal::ZERO,
                low: Decimal::ZERO,
                close: Decimal::from_str(price).unwrap(),
                volume: Decimal::ZERO,
            })
            .collect()
    }

    #[test]
    fn test_sma_calculation() {
        let prices = vec!["10", "12", "14", "16", "18"];
        let data = create_ohlcv(prices);
        let sma = SMA::new(3);
        let result = sma.calculate(&data).unwrap();

        // period 3
        // idx 0,1: None
        // idx 2: (10+12+14)/3 = 12
        // idx 3: (12+14+16)/3 = 14
        // idx 4: (14+16+18)/3 = 16

        assert_eq!(result[0], None);
        assert_eq!(result[1], None);
        assert_eq!(result[2], Some(Decimal::from_str("12").unwrap()));
        assert_eq!(result[3], Some(Decimal::from_str("14").unwrap()));
        assert_eq!(result[4], Some(Decimal::from_str("16").unwrap()));
    }

    #[test]
    fn test_rsi_calculation() {
        // RSI calculation is more complex, just verifying it runs and produces Some after period
        let mut prices = Vec::new();
        for i in 0..20 {
            if i % 2 == 0 {
                prices.push("10");
            } else {
                prices.push("12");
            }
        }
        let data = create_ohlcv(prices);
        let rsi = RSI::new(14);
        let result = rsi.calculate(&data).unwrap();

        assert_eq!(result.len(), 20);
        // First 14 should be None
        for i in 0..14 {
            assert_eq!(result[i], None, "Index {} should be None", i);
        }
        // subsequent should be Some
        for i in 14..20 {
            assert!(result[i].is_some(), "Index {} should be Some", i);
        }
    }
}

pub struct RSI {
    period: usize,
}

impl RSI {
    pub fn new(period: usize) -> Self {
        Self { period }
    }

    pub fn calculate(&self, data: &[OHLCV]) -> Result<Vec<Option<Decimal>>> {
        if data.len() < self.period + 1 {
            return Ok(vec![None; data.len()]);
        }

        let mut result = vec![None; self.period];
        let mut gains = vec![Decimal::ZERO; data.len()];
        let mut losses = vec![Decimal::ZERO; data.len()];

        for i in 1..data.len() {
            let change = data[i].close - data[i - 1].close;
            if change >= Decimal::ZERO {
                gains[i] = change;
                losses[i] = Decimal::ZERO;
            } else {
                gains[i] = Decimal::ZERO;
                losses[i] = change.abs();
            }
        }

        let avg_gain: Decimal = gains[1..=self.period].iter().sum::<Decimal>() / Decimal::from(self.period as u64);
        let avg_loss: Decimal = losses[1..=self.period].iter().sum::<Decimal>() / Decimal::from(self.period as u64);

        let mut prev_avg_gain = avg_gain;
        let mut prev_avg_loss = avg_loss;

        for i in self.period..data.len() {
            let avg_gain =
                (prev_avg_gain * Decimal::from(self.period as u64 - 1) + gains[i]) / Decimal::from(self.period as u64);
            let avg_loss =
                (prev_avg_loss * Decimal::from(self.period as u64 - 1) + losses[i]) / Decimal::from(self.period as u64);

            let rs = if avg_loss == Decimal::ZERO {
                Decimal::from(100)
            } else {
                avg_gain / avg_loss
            };

            result.push(Some(Decimal::from(100) - (Decimal::from(100) / (Decimal::ONE + rs))));

            prev_avg_gain = avg_gain;
            prev_avg_loss = avg_loss;
        }

        Ok(result)
    }
}

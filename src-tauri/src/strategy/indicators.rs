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
            let sum: Decimal = data[i - self.period + 1..=i].iter().map(|c| c.close).sum();
            result.push(Some(sum / Decimal::from(self.period as u64)));
        }

        Ok(result)
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

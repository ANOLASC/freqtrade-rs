use crate::backtest::BacktestConfig;
use crate::backtest::BacktestEngine;
use crate::error::Result;
use crate::optimize::{HyperoptParams, HyperoptValue};
use crate::persistence::Repository;
use crate::types::BacktestResult;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HyperoptConfig {
    pub epochs: usize,
    pub spaces: Vec<String>,
    pub strategy: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HyperoptEpoch {
    pub epoch: usize,
    pub params: HyperoptParams,
    pub result: Option<BacktestResult>,
    pub loss: f64,
    pub is_best: bool,
}

pub struct Hyperopt {
    config: HyperoptConfig,
}

impl Hyperopt {
    pub fn new(_repository: Arc<Repository>, config: HyperoptConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<OptimizerResult> {
        let mut results = Vec::new();
        let mut best_loss = f64::MAX;
        let mut best_epoch = 0;
        let mut best_params = HyperoptParams::new();

        for epoch in 0..self.config.epochs {
            let params = self.generate_random_params()?;

            let result = match self.run_backtest(&params).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Epoch {} failed: {}", epoch, e);
                    continue;
                }
            };

            let loss = self.calculate_loss(&result);

            let is_best = loss < best_loss;
            if is_best {
                best_loss = loss;
                best_epoch = epoch;
                best_params = params.clone();
            }

            results.push(HyperoptEpoch {
                epoch,
                loss,
                params,
                result: Some(result.clone()),
                is_best,
            });

            if epoch % 10 == 0 {
                eprintln!("Epoch {}: Loss = {}", epoch, loss);
            }
        }

        Ok(OptimizerResult {
            best_epoch,
            best_loss,
            best_params,
            epoch_results: results,
        })
    }

    fn generate_random_params(&self) -> Result<HyperoptParams> {
        let mut params = HyperoptParams::new();

        for space in &self.config.spaces {
            let value = match space.as_str() {
                "buy" => HyperoptValue::Float(1.0),
                "sell" => HyperoptValue::Float(1.0),
                "roi" => HyperoptValue::Float(0.5),
                "stoploss" => HyperoptValue::Float(-0.05),
                _ => HyperoptValue::Float(0.0),
            };
            params.insert(space.clone(), value);
        }

        Ok(params)
    }

    async fn run_backtest(&self, _params: &HyperoptParams) -> Result<BacktestResult> {
        let config = BacktestConfig {
            start_date: chrono::Utc::now() - chrono::Duration::days(30),
            end_date: chrono::Utc::now(),
            stake_amount: 100.0,
            commission: 0.0,
        };

        let data = Vec::new();
        let strategy = Arc::new(StubStrategy);
        let mut engine = BacktestEngine::new(config, strategy, data);
        engine.run().await
    }

    fn calculate_loss(&self, result: &BacktestResult) -> f64 {
        -result.sharpe_ratio
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizerResult {
    pub best_epoch: usize,
    pub best_loss: f64,
    pub best_params: HyperoptParams,
    pub epoch_results: Vec<HyperoptEpoch>,
}

struct StubStrategy;

#[async_trait::async_trait]
impl crate::strategy::Strategy for StubStrategy {
    fn name(&self) -> &str {
        "Stub"
    }

    fn timeframes(&self) -> &[crate::types::Timeframe] {
        &[crate::types::Timeframe::OneHour]
    }

    async fn populate_indicators(&mut self, _data: &mut Vec<crate::types::OHLCV>) -> Result<()> {
        Ok(())
    }

    async fn populate_buy_trend(
        &self,
        _data: &[crate::types::OHLCV],
    ) -> Result<Vec<crate::types::Signal>> {
        Ok(vec![])
    }

    async fn populate_sell_trend(
        &self,
        _data: &[crate::types::OHLCV],
    ) -> Result<Vec<crate::types::Signal>> {
        Ok(vec![])
    }
}

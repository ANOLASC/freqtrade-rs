use crate::backtest::BacktestEngine;
use crate::error::Result;
use crate::optimize::{HyperoptParams, HyperoptValue};
use crate::types::BacktestResult;

#[derive(Debug, Clone, Copy)]
pub struct OptimizerConfig {
    pub epochs: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizerResult {
    pub best_epoch: usize,
    pub best_loss: f64,
    pub best_params: HyperoptParams,
    pub epoch_results: Vec<EpochResult>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EpochResult {
    pub epoch: usize,
    pub loss: f64,
    pub params: HyperoptParams,
}

pub trait Optimizer: Send + Sync {
    fn name(&self) -> &str;
    fn optimize(&self, config: &OptimizerConfig, _loss_fn: ()) -> Result<OptimizerResult>;
}

pub struct RandomOptimizer;

impl RandomOptimizer {
    pub fn new() -> Self {
        Self
    }
}

impl Optimizer for RandomOptimizer {
    fn name(&self) -> &str {
        "Random"
    }

    fn optimize(&self, config: &OptimizerConfig, _loss_fn: ()) -> Result<OptimizerResult> {
        let mut results = Vec::new();
        let mut best_loss = f64::MAX;
        let mut best_params = HyperoptParams::new();

        for epoch in 0..config.epochs {
            let params = HyperoptParams::new();
            let params_clone = params.clone();
            results.push(EpochResult {
                epoch,
                loss: 0.0,
                params,
            });

            if 0.0 < best_loss {
                best_loss = 0.0;
                best_params = params_clone;
            }
        }

        Ok(OptimizerResult {
            best_epoch: 0,
            best_loss,
            best_params,
            epoch_results: results,
        })
    }
}

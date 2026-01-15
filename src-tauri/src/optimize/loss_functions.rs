use crate::types::BacktestResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LossFunctionType {
    Sharpe,
    Sortino,
    Calmar,
    ProfitFactor,
    Custom(String),
}

impl LossFunctionType {
    pub fn as_str(&self) -> &str {
        match self {
            LossFunctionType::Sharpe => "Sharpe",
            LossFunctionType::Sortino => "Sortino",
            LossFunctionType::Calmar => "Calmar",
            LossFunctionType::ProfitFactor => "ProfitFactor",
            LossFunctionType::Custom(s) => s,
        }
    }
}

pub trait LossFunction: Send + Sync {
    fn name(&self) -> &str;
    fn calculate(&self, result: &BacktestResult) -> f64;
}

pub struct SharpeLoss;
pub struct SortinoLoss;
pub struct CalmarLoss;
pub struct ProfitFactorLoss;

impl LossFunction for SharpeLoss {
    fn name(&self) -> &str {
        "Sharpe"
    }

    fn calculate(&self, result: &BacktestResult) -> f64 {
        -result.sharpe_ratio
    }
}

impl LossFunction for SortinoLoss {
    fn name(&self) -> &str {
        "Sortino"
    }

    fn calculate(&self, result: &BacktestResult) -> f64 {
        if result.sharpe_ratio > 0.0 {
            -result.sharpe_ratio
        } else {
            999.0
        }
    }
}

impl LossFunction for CalmarLoss {
    fn name(&self) -> &str {
        "Calmar"
    }

    fn calculate(&self, result: &BacktestResult) -> f64 {
        if result.sharpe_ratio > 0.0 {
            -result.sharpe_ratio / result.max_drawdown.abs()
        } else {
            999.0
        }
    }
}

impl LossFunction for ProfitFactorLoss {
    fn name(&self) -> &str {
        "ProfitFactor"
    }

    fn calculate(&self, result: &BacktestResult) -> f64 {
        if result.profit_factor > 0.0 {
            -result.profit_factor
        } else {
            999.0
        }
    }
}

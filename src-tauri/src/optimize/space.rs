use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HyperoptSpace {
    Buy(String),
    Sell(String),
    Protection(String),
    Trailing(String),
    ROI(String),
    Stoploss(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HyperoptValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl HyperoptValue {
    pub fn as_float(&self) -> Option<f64> {
        match self {
            HyperoptValue::Float(v) => Some(*v),
            HyperoptValue::Int(v) => Some(*v as f64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSpace {
    pub name: String,
    pub space: Vec<HyperoptSpace>,
}

impl ParameterSpace {
    pub fn new(name: String) -> Self {
        Self { name, space: Vec::new() }
    }
    
    pub fn with_spaces(mut self, spaces: Vec<HyperoptSpace>) -> Self {
        self.space = spaces;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperoptParams {
    pub params: std::collections::HashMap<String, HyperoptValue>,
}

impl HyperoptParams {
    pub fn new() -> Self {
        Self {
            params: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: String, value: HyperoptValue) {
        self.params.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&HyperoptValue> {
        self.params.get(key)
    }
}

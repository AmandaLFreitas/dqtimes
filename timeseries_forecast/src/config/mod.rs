use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LinearConfig {
    pub horizon: usize,
    pub include_intercept: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SMAConfig {
    pub window: usize,
    pub horizon: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EWMAConfig {
    pub alpha: f64,
    pub horizon: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WMAConfig {
    pub weights: Vec<f64>,
    pub horizon: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArimaConfig {
    pub p: usize,
    pub d: usize,
    pub q: usize,
    pub horizon: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeuralConfig {
    pub horizon: usize,
    pub hidden_layers: Vec<usize>,
}

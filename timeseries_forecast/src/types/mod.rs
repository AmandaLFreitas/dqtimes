use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimeSeries {
    pub points: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationMetrics {
    pub mae: Option<f64>,
    pub mse: Option<f64>,
    pub rmse: Option<f64>,
    pub mape: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForecastOutput {
    pub predicted: Vec<f64>,
    pub error_margin: Option<f64>,
    pub model: ModelInfo,
    pub performance: EvaluationMetrics,
}

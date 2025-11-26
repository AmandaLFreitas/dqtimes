use crate::config::NeuralConfig;
use crate::errors::ForecastError;
use crate::types::{ForecastOutput, TimeSeries};

pub fn forecast_neural(
    series: &TimeSeries,
    config: &NeuralConfig,
) -> Result<ForecastOutput, ForecastError> {
    let _ = (series, config);
    Err(ForecastError::NotImplemented)
}

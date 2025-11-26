use crate::config::LinearConfig;
use crate::errors::ForecastError;
use crate::types::{ForecastOutput, TimeSeries};

pub fn forecast_linear(
    series: &TimeSeries,
    config: &LinearConfig,
) -> Result<ForecastOutput, ForecastError> {
    let _ = (series, config);
    Err(ForecastError::NotImplemented)
}

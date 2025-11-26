use crate::config::{EWMAConfig, SMAConfig, WMAConfig};
use crate::errors::ForecastError;
use crate::types::{ForecastOutput, TimeSeries};

pub fn forecast_sma(
    series: &TimeSeries,
    config: &SMAConfig,
) -> Result<ForecastOutput, ForecastError> {
    let _ = (series, config);
    Err(ForecastError::NotImplemented)
}

pub fn forecast_ewma(
    series: &TimeSeries,
    config: &EWMAConfig,
) -> Result<ForecastOutput, ForecastError> {
    let _ = (series, config);
    Err(ForecastError::NotImplemented)
}

pub fn forecast_wma(
    series: &TimeSeries,
    config: &WMAConfig,
) -> Result<ForecastOutput, ForecastError> {
    let _ = (series, config);
    Err(ForecastError::NotImplemented)
}

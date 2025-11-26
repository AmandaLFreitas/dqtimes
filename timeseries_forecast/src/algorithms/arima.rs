use crate::config::ArimaConfig;
use crate::errors::ForecastError;
use crate::types::{ForecastOutput, TimeSeries};

pub fn forecast_arima(
    series: &TimeSeries,
    config: &ArimaConfig,
) -> Result<ForecastOutput, ForecastError> {
    let _ = (series, config);
    Err(ForecastError::NotImplemented)
}

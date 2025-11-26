use crate::errors::ForecastError;
use crate::types::TimeSeries;

pub fn validate_series(series: &TimeSeries) -> Result<(), ForecastError> {
    if series.points.is_empty() {
        return Err(ForecastError::InvalidInput("série temporal vazia".into()));
    }
    if series
        .points
        .iter()
        .any(|p| p.value.is_nan() || p.value.is_infinite())
    {
        return Err(ForecastError::InvalidInput(
            "valores inválidos (NaN/Inf)".into(),
        ));
    }
    Ok(())
}

pub fn sort_series(series: &mut TimeSeries) {
    series.points.sort_by_key(|p| p.timestamp);
}

use chrono::Utc;
use timeseries_forecast::types::{TimeSeries, TimeSeriesPoint};
use timeseries_forecast::utils::validate_series;

#[test]
fn validate_empty_series_should_fail() {
    let ts = TimeSeries { points: vec![] };
    assert!(validate_series(&ts).is_err());
}

#[test]
fn validate_non_empty_series_should_pass() {
    let ts = TimeSeries {
        points: vec![TimeSeriesPoint {
            timestamp: Utc::now(),
            value: 1.0,
        }],
    };
    assert!(validate_series(&ts).is_ok());
}

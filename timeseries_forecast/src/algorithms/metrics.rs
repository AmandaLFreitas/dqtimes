use crate::types::EvaluationMetrics;

pub fn compute_metrics(predicted: &[f64], actual: &[f64]) -> EvaluationMetrics {
    let _ = (predicted, actual);
    EvaluationMetrics::default()
}

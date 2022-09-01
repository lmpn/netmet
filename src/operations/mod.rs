use std::collections::VecDeque;

pub fn mean(metrics: &VecDeque<super::types::Metric>) -> f64 {
    let val = metrics.iter().map(|(_, w)| w).sum::<u64>() as f64;
    val / 50.0
}

use crate::bench::BenchmarkResult;

/// A detected performance regression
#[derive(Debug, Clone, serde::Serialize)]
pub struct Regression {
    pub name: String,
    pub baseline_ns: f64,
    pub current_ns: f64,
    pub change_pct: f64,
    pub is_regression: bool,
}

/// Detects regressions by comparing benchmark results
pub struct RegressionDetector {
    threshold_pct: f64,
}

impl RegressionDetector {
    pub fn new(threshold_pct: f64) -> Self {
        RegressionDetector { threshold_pct }
    }

    pub fn compare(&self, baseline: &[BenchmarkResult], current: &[BenchmarkResult]) -> Vec<Regression> {
        let mut regressions = Vec::new();
        for b in baseline {
            if let Some(c) = current.iter().find(|c| c.name == b.name) {
                let change_pct = ((c.mean_ns - b.mean_ns) / b.mean_ns) * 100.0;
                regressions.push(Regression {
                    name: b.name.clone(),
                    baseline_ns: b.mean_ns,
                    current_ns: c.mean_ns,
                    change_pct,
                    is_regression: change_pct > self.threshold_pct,
                });
            }
        }
        regressions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(name: &str, mean: f64) -> BenchmarkResult {
        BenchmarkResult {
            name: name.to_string(), iterations: 100, mean_ns: mean,
            median_ns: mean, std_dev_ns: 0.0, p95_ns: mean, p99_ns: mean,
            throughput_ops_per_sec: 1e9 / mean,
        }
    }

    #[test]
    fn test_no_regression() {
        let det = RegressionDetector::new(10.0);
        let baseline = vec![make_result("a", 100.0)];
        let current = vec![make_result("a", 105.0)];
        let results = det.compare(&baseline, &current);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_regression);
    }

    #[test]
    fn test_regression_detected() {
        let det = RegressionDetector::new(10.0);
        let baseline = vec![make_result("a", 100.0)];
        let current = vec![make_result("a", 120.0)];
        let results = det.compare(&baseline, &current);
        assert!(results[0].is_regression);
        assert!((results[0].change_pct - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_improvement() {
        let det = RegressionDetector::new(10.0);
        let baseline = vec![make_result("a", 100.0)];
        let current = vec![make_result("a", 80.0)];
        let results = det.compare(&baseline, &current);
        assert!(!results[0].is_regression);
        assert!(results[0].change_pct < 0.0);
    }

    #[test]
    fn test_empty_baseline() {
        let det = RegressionDetector::new(10.0);
        let results = det.compare(&[], &[make_result("a", 100.0)]);
        assert!(results.is_empty());
    }
}

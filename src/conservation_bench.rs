use crate::bench::{Benchmark, BenchmarkResult};

pub fn bench_budget_creation() -> BenchmarkResult {
    Benchmark::new("budget_creation").run(10000, || {
        let _gamma: u64 = 143;
        let _eta: u64 = 82;
        let _total: u64 = _gamma + _eta;
    })
}

pub fn bench_conservation_check() -> BenchmarkResult {
    let gamma: u64 = 143;
    let eta: u64 = 82;
    Benchmark::new("conservation_check").run(10000, || {
        let _ = gamma + eta == 225;
    })
}

pub fn bench_fleet_1000() -> BenchmarkResult {
    let budgets: Vec<(u64, u64)> = (0..1000).map(|i| (i, i * 2)).collect();
    Benchmark::new("fleet_sum_1000").run(100, || {
        let _gamma_total: u64 = budgets.iter().map(|(g, _)| g).sum();
        let _eta_total: u64 = budgets.iter().map(|(_, e)| e).sum();
    })
}

pub fn bench_fleet_10000() -> BenchmarkResult {
    let budgets: Vec<(u64, u64)> = (0..10000).map(|i| (i, i * 2)).collect();
    Benchmark::new("fleet_sum_10000").run(100, || {
        let _gamma_total: u64 = budgets.iter().map(|(g, _)| g).sum();
        let _eta_total: u64 = budgets.iter().map(|(_, e)| e).sum();
    })
}

pub fn run_all_conservation_benches() -> Vec<BenchmarkResult> {
    vec![
        bench_budget_creation(),
        bench_conservation_check(),
        bench_fleet_1000(),
        bench_fleet_10000(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_creation_bench() {
        let result = bench_budget_creation();
        assert!(result.mean_ns > 0.0);
        assert!(result.throughput_ops_per_sec > 0.0);
    }

    #[test]
    fn test_conservation_check_bench() {
        let result = bench_conservation_check();
        assert!(result.mean_ns > 0.0);
    }

    #[test]
    fn test_fleet_1000_bench() {
        let result = bench_fleet_1000();
        assert!(result.mean_ns > 0.0);
    }

    #[test]
    fn test_run_all() {
        let results = run_all_conservation_benches();
        assert_eq!(results.len(), 4);
    }
}

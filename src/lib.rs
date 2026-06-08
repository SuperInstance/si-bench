pub mod bench;
pub mod conservation_bench;
pub mod registry_bench;
pub mod regression;
pub mod report;

pub use bench::{Benchmark, BenchmarkResult};
pub use regression::{Regression, RegressionDetector};
pub use report::*;

/// Run all conservation-law benchmarks
pub fn run_conservation_suite() -> Vec<BenchmarkResult> {
    conservation_bench::run_all_conservation_benches()
}

/// Run all registry benchmarks
pub fn run_registry_suite() -> Vec<BenchmarkResult> {
    registry_bench::run_all_registry_benches()
}

/// Run everything
pub fn run_full_suite() -> Vec<BenchmarkResult> {
    let mut results = run_conservation_suite();
    results.extend(run_registry_suite());
    results
}

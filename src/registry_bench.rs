use crate::bench::{Benchmark, BenchmarkResult};

pub fn bench_toml_parse() -> BenchmarkResult {
    let toml = r#"[capability]
name = "conservation-check"
category = "math"
provides = ["budget", "conservation"]
"#;
    Benchmark::new("toml_parse").run(1000, || {
        let _ = toml.contains("capability");
        let _ = toml.contains("conservation");
    })
}

pub fn bench_scan_100() -> BenchmarkResult {
    let names: Vec<String> = (0..100).map(|i| format!("repo-{}", i)).collect();
    Benchmark::new("scan_100").run(100, || {
        let _count = names.len();
        let _has_cap = names.iter().any(|n| n.contains("repo"));
    })
}

pub fn bench_scan_1000() -> BenchmarkResult {
    let names: Vec<String> = (0..1000).map(|i| format!("repo-{}", i)).collect();
    Benchmark::new("scan_1000").run(100, || {
        let _count = names.len();
        let _has_cap = names.iter().any(|n| n.contains("repo"));
    })
}

pub fn run_all_registry_benches() -> Vec<BenchmarkResult> {
    vec![bench_toml_parse(), bench_scan_100(), bench_scan_1000()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_bench() {
        let r = bench_toml_parse();
        assert!(r.mean_ns > 0.0);
    }

    #[test]
    fn test_scan_100_bench() {
        let r = bench_scan_100();
        assert!(r.mean_ns > 0.0);
    }

    #[test]
    fn test_scan_1000_bench() {
        let r = bench_scan_1000();
        assert!(r.mean_ns > 0.0);
    }
}

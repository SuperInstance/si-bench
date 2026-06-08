# si-bench

Fleet-wide benchmarking and performance regression testing for [SuperInstance](https://github.com/SuperInstance).

## What It Does

si-bench measures the performance of conservation-law operations across the fleet and detects regressions between runs.

```
use si_bench::{Benchmark, RegressionDetector, format_table};

// Run a benchmark
let bench = Benchmark::new("conservation_check");
let result = bench.run(10_000, || {
    let gamma: u64 = 143;
    let eta: u64 = 82;
    let _ = gamma + eta == 225;
});

println!("Mean: {:.1}ns, P99: {:.1}ns, Throughput: {:.0}/s",
    result.mean_ns, result.p99_ns, result.throughput_ops_per_sec);

// Detect regressions
let detector = RegressionDetector::new(10.0); // 10% threshold
let regressions = detector.compare(&baseline_results, &current_results);
for r in &regressions {
    if r.is_regression {
        println!("⚠️ {} regressed by {:.1}%", r.name, r.change_pct);
    }
}

// Format results
println!("{}", format_table(&results));
```

## Built-in Benchmarks

| Benchmark | What it measures |
|-----------|-----------------|
| `budget_creation` | Single budget object creation |
| `conservation_check` | γ + η == total invariant check |
| `fleet_sum_1000` | Sum 1,000 budgets |
| `fleet_sum_10000` | Sum 10,000 budgets |
| `toml_parse` | CAPABILITY.toml parsing |
| `scan_100` | Scan 100 repos |
| `scan_1000` | Scan 1,000 repos |

## Modules

- `bench` — Core benchmarking with statistical analysis (mean, median, std_dev, p95, p99)
- `conservation_bench` — Pre-built conservation-law benchmarks
- `registry_bench` — Pre-built registry/scan benchmarks
- `regression` — Regression detection with configurable threshold
- `report` — Output formatting (table, markdown, JSON)

## Architecture

```
si-bench
├── bench.rs              → Benchmark, BenchmarkResult (statistics)
├── conservation_bench.rs → Fleet-wide conservation benchmarks
├── registry_bench.rs     → Registry/scan benchmarks
├── regression.rs         → RegressionDetector, Regression
└── report.rs             → format_table, format_markdown, format_json
```

## Conservation Law

All benchmarks enforce γ + η = total_budget. The regression detector flags when performance degrades beyond a threshold percentage, catching conservation-law violations early.

## Install

```toml
[dependencies]
si-bench = "0.1"
```

## License

MIT

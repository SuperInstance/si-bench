use crate::bench::BenchmarkResult;

pub fn format_table(results: &[BenchmarkResult]) -> String {
    let mut out = String::new();
    out.push_str(&format!("{:<30} {:>10} {:>12} {:>12} {:>12} {:>15}\n",
        "Benchmark", "Iters", "Mean (ns)", "Median (ns)", "P99 (ns)", "Throughput"));
    out.push_str(&"-".repeat(95));
    out.push('\n');
    for r in results {
        out.push_str(&format!("{:<30} {:>10} {:>12.1} {:>12.1} {:>12.1} {:>12.0}/s\n",
            r.name, r.iterations, r.mean_ns, r.median_ns, r.p99_ns, r.throughput_ops_per_sec));
    }
    out
}

pub fn format_markdown(results: &[BenchmarkResult]) -> String {
    let mut out = String::from("| Benchmark | Iterations | Mean (ns) | Median (ns) | P99 (ns) | Throughput |\n");
    out.push_str("|-----------|-----------|-----------|------------|----------|------------|\n");
    for r in results {
        out.push_str(&format!("| {} | {} | {:.1} | {:.1} | {:.1} | {:.0}/s |\n",
            r.name, r.iterations, r.mean_ns, r.median_ns, r.p99_ns, r.throughput_ops_per_sec));
    }
    out
}

pub fn format_json(results: &[BenchmarkResult]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(name: &str) -> BenchmarkResult {
        BenchmarkResult {
            name: name.to_string(), iterations: 100, mean_ns: 500.0,
            median_ns: 480.0, std_dev_ns: 50.0, p95_ns: 600.0, p99_ns: 700.0,
            throughput_ops_per_sec: 2_000_000.0,
        }
    }

    #[test]
    fn test_format_table() {
        let results = vec![make_result("test_bench")];
        let table = format_table(&results);
        assert!(table.contains("test_bench"));
        assert!(table.contains("500"));
    }

    #[test]
    fn test_format_markdown() {
        let results = vec![make_result("test_bench")];
        let md = format_markdown(&results);
        assert!(md.contains("| test_bench"));
        assert!(md.contains("|-----------|"));
    }

    #[test]
    fn test_format_json() {
        let results = vec![make_result("test_bench")];
        let json = format_json(&results);
        assert!(json.contains("\"name\": \"test_bench\""));
        assert!(json.contains("\"mean_ns\": 500.0"));
    }
}

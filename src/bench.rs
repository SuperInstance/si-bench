use std::time::Instant;

/// Result of a single benchmark run
#[derive(Debug, Clone, serde::Serialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub std_dev_ns: f64,
    pub p95_ns: f64,
    pub p99_ns: f64,
    pub throughput_ops_per_sec: f64,
}

/// A named benchmark
pub struct Benchmark {
    name: String,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        Benchmark { name: name.to_string() }
    }

    /// Run a closure many times and collect statistics
    pub fn run<F: FnMut()>(&self, iterations: usize, mut f: F) -> BenchmarkResult {
        let mut samples: Vec<u64> = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            let start = Instant::now();
            f();
            samples.push(start.elapsed().as_nanos() as u64);
        }
        
        samples.sort();
        
        let mean_ns = samples.iter().sum::<u64>() as f64 / iterations as f64;
        let median_ns = samples[iterations / 2] as f64;
        let variance: f64 = samples.iter()
            .map(|&s| { let d = s as f64 - mean_ns; d * d })
            .sum::<f64>() / iterations as f64;
        let std_dev_ns = variance.sqrt();
        let p95_ns = samples[(iterations as f64 * 0.95) as usize] as f64;
        let p99_ns = samples[(iterations as f64 * 0.99) as usize] as f64;
        let throughput = if mean_ns > 0.0 { 1_000_000_000.0 / mean_ns } else { 0.0 };
        
        BenchmarkResult {
            name: self.name.clone(),
            iterations,
            mean_ns,
            median_ns,
            std_dev_ns,
            p95_ns,
            p99_ns,
            throughput_ops_per_sec: throughput,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_new() {
        let b = Benchmark::new("test");
        assert_eq!(b.name, "test");
    }

    #[test]
    fn test_benchmark_run() {
        let b = Benchmark::new("add");
        let result = b.run(100, || { let _ = 1 + 1; });
        assert_eq!(result.name, "add");
        assert_eq!(result.iterations, 100);
        assert!(result.mean_ns > 0.0);
        assert!(result.median_ns > 0.0);
        assert!(result.throughput_ops_per_sec > 0.0);
    }

    #[test]
    fn test_benchmark_p95_p99() {
        let b = Benchmark::new("p_test");
        let result = b.run(1000, || { let _ = 2 * 3; });
        assert!(result.p95_ns > 0.0);
        assert!(result.p99_ns >= result.p95_ns * 0.5);
    }
}

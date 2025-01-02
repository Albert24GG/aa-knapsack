use core::str;
use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
    time::Duration,
};

use criterion::{black_box, Criterion};
use knapsack::{KnapsackInput, KnapsackSolver};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct ConfidenceInterval {
    confidence_level: f64,
    #[serde(
        deserialize_with = "deserialize_nanos",
        serialize_with = "serialize_nanos"
    )]
    lower_bound: Duration,
    #[serde(
        deserialize_with = "deserialize_nanos",
        serialize_with = "serialize_nanos"
    )]
    upper_bound: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct MetricEstimation {
    confidence_interval: ConfidenceInterval,
    #[serde(
        deserialize_with = "deserialize_nanos",
        serialize_with = "serialize_nanos"
    )]
    point_estimate: Duration,
    standard_error: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnapsackBenchResult {
    mean: MetricEstimation,
    median: MetricEstimation,
    std_dev: MetricEstimation,
}

#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Deserialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

fn deserialize_nanos<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let nanos = f64::deserialize(deserializer)?;
    Ok(Duration::from_nanos(nanos.floor() as u64))
}

fn serialize_nanos<S>(nanos: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(nanos.as_nanos() as u64)
}

fn extract_results(path: impl AsRef<Path>) -> Result<KnapsackBenchResult, BenchmarkError> {
    let results_file = File::open(path)?;
    let reader = BufReader::new(results_file);
    let results = serde_json::from_reader(reader)?;

    Ok(results)
}

pub fn run_benchmark(
    solver: &dyn KnapsackSolver,
    input: &KnapsackInput,
) -> Result<KnapsackBenchResult, BenchmarkError> {
    let mut criterion = Criterion::default().without_plots();

    criterion.bench_function(solver.method().into(), |b| {
        b.iter(|| solver.solve(black_box(input)))
    });

    let results_path = format!(
        "{}/target/criterion/{}/new/estimates.json",
        env!("CARGO_MANIFEST_DIR"),
        solver.method().as_ref()
    );

    extract_results(results_path)
}

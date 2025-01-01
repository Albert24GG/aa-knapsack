use clap::{Parser, Subcommand, ValueEnum};
use criterion::{black_box, Criterion};
use knapsack::{
    BktSolver, DpSolver, FptasDpSolver, KnapsackInput, KnapsackItem, KnapsackMethod, KnapsackSolver,
};
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Duration;
use text_io::scan;

#[derive(Debug, Parser)]
struct CommandArgs {
    #[arg(short, long, value_name = "TEST_FILE", value_hint = clap::ValueHint::FilePath)]
    input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE", default_value = "out.json")]
    output_file: PathBuf,

    #[arg()]
    /// Action to perform
    action: KnapsackAction,

    /// Method used for solving the problem
    #[clap(subcommand)]
    method: KnapsackMethodCmd,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum KnapsackAction {
    Run,
    Benchmark,
}

#[derive(Debug, Subcommand, Hash, PartialEq, Eq)]
enum KnapsackMethodCmd {
    Dp,
    Bkt,
    Fptas {
        #[arg(default_value_t = 1)]
        granularity: u32,
    },
}

#[derive(Debug, Serialize)]
pub struct KnapsackBenchResult {
    median: Duration,
    mean: Duration,
    std_dev: Duration,
    max: Duration,
    min: Duration,
}

lazy_static! {
    static ref METHOD_MAPPER: HashMap<KnapsackMethod, Box<dyn KnapsackSolver>> = {
        let mut m = HashMap::new();
        m.insert(
            KnapsackMethod::Dp,
            Box::new(DpSolver) as Box<dyn KnapsackSolver>,
        );
        m.insert(
            KnapsackMethod::Bkt,
            Box::new(BktSolver) as Box<dyn KnapsackSolver>,
        );
        m.insert(
            KnapsackMethod::Fptas,
            Box::new(FptasDpSolver) as Box<dyn KnapsackSolver>,
        );
        m
    };
}

fn parse_input(args: &CommandArgs) -> KnapsackInput {
    let file = File::open(&args.input_file).unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty());

    let n: usize;
    {
        let line = lines
            .next()
            .expect("Missing number of items (n) in the input");
        scan!(line.bytes() => "{}", n);
    }

    let capacity: u32;
    {
        let line = lines.next().expect("Missing capacity in the input");
        scan!(line.bytes() => "{}", capacity);
    }

    // Parse items
    let mut items = Vec::with_capacity(n);
    for (index, line) in lines.take(n).enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            panic!("Invalid line at item {}: {:?}", index + 1, line);
        }

        let value: u32 = parts[0].parse().expect("Failed to parse value");
        let weight: u32 = parts[1].parse().expect("Failed to parse weight");

        items.push(KnapsackItem::new(weight, value));
    }

    let granularity = match args.method {
        KnapsackMethodCmd::Fptas { granularity } => Some(granularity),
        _ => None,
    };

    KnapsackInput::new(items, capacity, granularity).unwrap()
}

fn get_method(method: &KnapsackMethodCmd) -> KnapsackMethod {
    match method {
        KnapsackMethodCmd::Dp => KnapsackMethod::Dp,
        KnapsackMethodCmd::Bkt => KnapsackMethod::Bkt,
        KnapsackMethodCmd::Fptas { .. } => KnapsackMethod::Fptas,
    }
}

fn get_solver(method: &KnapsackMethodCmd) -> Option<&dyn KnapsackSolver> {
    let method = get_method(method);
    METHOD_MAPPER
        .get(&method)
        .map(|boxed_trait| boxed_trait.as_ref())
}

fn process_duration(durations: &[Duration]) -> KnapsackBenchResult {
    let mut durations = durations.to_vec();
    durations.sort();

    let len = durations.len();
    let median = durations[len / 2];
    let mean = durations.iter().sum::<Duration>() / len as u32;
    let std_dev = durations
        .iter()
        .map(|d| (d.as_nanos() as f64 - mean.as_nanos() as f64).powi(2))
        .sum::<f64>()
        .sqrt() as u64;
    let max = *durations.last().unwrap();
    let min = *durations.first().unwrap();

    KnapsackBenchResult {
        median,
        mean,
        std_dev: Duration::from_nanos(std_dev),
        max,
        min,
    }
}

fn run_benchmark(solver: &dyn KnapsackSolver, input: &KnapsackInput) -> KnapsackBenchResult {
    let mut criterion = Criterion::default().without_plots();

    let mut durations = Vec::<Duration>::new();

    criterion.bench_function(solver.method().into(), |b| {
        b.iter(|| {
            let start = std::time::Instant::now();
            solver.solve(black_box(input));
            let elapsed = start.elapsed();
            durations.push(elapsed);
        })
    });

    process_duration(&durations)
}

fn main() {
    let args = CommandArgs::parse();

    let input = parse_input(&args);
    let solver = get_solver(&args.method).unwrap();

    let output = match args.action {
        KnapsackAction::Run => {
            let solution = solver.solve(&input);
            serde_json::to_value(&solution).unwrap()
        }
        KnapsackAction::Benchmark => {
            let result = run_benchmark(solver, &input);
            serde_json::to_value(&result).unwrap()
        }
    };

    let file = File::create(args.output_file.clone()).ok();
    match file {
        Some(file) => serde_json::to_writer(file, &output).unwrap(),
        None => {
            println!(
                "Failed to write output to \"{}\":\n {}",
                args.output_file.display(),
                output
            )
        }
    }
}

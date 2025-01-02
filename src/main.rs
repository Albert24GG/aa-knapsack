mod benchmark;

use benchmark::run_benchmark;
use clap::{Parser, Subcommand, ValueEnum};
use knapsack::{
    BktSolver, DpSolver, FptasDpSolver, KnapsackInput, KnapsackItem, KnapsackMethod, KnapsackSolver,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
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
            let result = run_benchmark(solver, &input).unwrap();
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

mod benchmark;

use benchmark::run_benchmark;
use clap::{Parser, ValueEnum};
use knapsack::{BktSolver, DpSolver, FptasDpSolver, KnapsackInput, KnapsackMethod, KnapsackSolver};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct CommandArgs {
    #[arg(short, long, value_name = "TEST_FILE", value_hint = clap::ValueHint::FilePath)]
    input_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE", value_hint = clap::ValueHint::FilePath, default_value = "out.json")]
    output_file: PathBuf,

    #[arg(short, long, default_value_t = 1)]
    /// Granularity for the FPTAS method. This is only used when the method is FPTAS.
    granularity: u32,

    #[arg()]
    /// Action to perform
    action: KnapsackAction,

    #[arg()]
    /// Method used for solving the problem
    method: KnapsackMethodCmd,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum KnapsackAction {
    Run,
    Benchmark,
}

#[derive(Debug, Clone, Copy, ValueEnum, Hash, PartialEq, Eq)]
enum KnapsackMethodCmd {
    Dp,
    Bkt,
    Fptas,
}

lazy_static! {
    static ref METHOD_MAPPER: HashMap<KnapsackMethod, &'static dyn KnapsackSolver> = {
        let mut m = HashMap::new();
        m.insert(KnapsackMethod::Dp, &DpSolver as &dyn KnapsackSolver);
        m.insert(KnapsackMethod::Bkt, &BktSolver as &dyn KnapsackSolver);
        m.insert(KnapsackMethod::Fptas, &FptasDpSolver as &dyn KnapsackSolver);
        m
    };
}

fn parse_input(args: &CommandArgs) -> KnapsackInput {
    let file = File::open(&args.input_file).unwrap();

    let reader = BufReader::new(file);

    let mut parsed_input = KnapsackInput::parse_input(reader).unwrap();
    parsed_input.set_granularity(args.granularity).unwrap();

    parsed_input
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
    METHOD_MAPPER.get(&method).copied()
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

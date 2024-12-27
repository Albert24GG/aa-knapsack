use clap::{Parser, Subcommand};
use knapsack::knapsack::{bkt::BktSolver, dp::DpSolver};
use knapsack::knapsack::{KnapsackInput, KnapsackItem, KnapsackSolver};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use text_io::scan;

#[derive(Debug, Parser)]
struct CommandArgs {
    #[arg(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    input_file: PathBuf,

    #[clap(subcommand)]
    cmd: KnapsackMethodCmd,
}

#[derive(Debug, Subcommand)]
enum KnapsackMethodCmd {
    Dp,
    Bkt,
    Fptas {
        #[arg(default_value_t = 1)]
        granularity: u32,
    },
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

    KnapsackInput::new(items, capacity, None).unwrap()
}

fn main() {
    let args = CommandArgs::parse();

    let input = parse_input(&args);

    let solution = match args.cmd {
        KnapsackMethodCmd::Dp => DpSolver::solve(&input),
        KnapsackMethodCmd::Bkt => BktSolver::solve(&input),
        KnapsackMethodCmd::Fptas { granularity } => unimplemented!(),
    };

    println!("Optimal value: {}", solution.total_value);
    println!(
        "Selected items: {}",
        solution
            .items
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

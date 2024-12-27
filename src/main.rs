use knapsack::knapsack::dp::DpSolver;
use knapsack::knapsack::{KnapsackInput, KnapsackItem, KnapsackSolver};
use std::io::{self, BufRead};

fn parse_input() -> KnapsackInput {
    let mut lines = io::stdin()
        .lock()
        .lines()
        .filter(|line| !line.as_ref().unwrap().is_empty());

    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse n");
    let capacity: u32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse capacity");

    let mut items = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();

        let mut parts = line.split_whitespace();

        let value: u32 = parts
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse value");
        let weight: u32 = parts
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse weight");

        items.push(KnapsackItem::new(weight, value));
    }

    KnapsackInput::new(items, capacity, None).unwrap()
}

fn main() {
    let solution = DpSolver::solve(&parse_input());

    println!("{}", solution.total_value);
    println!(
        "{}",
        solution
            .items
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

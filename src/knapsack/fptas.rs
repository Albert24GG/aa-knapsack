use super::dp::DpSolver;
use super::{KnapsackInput, KnapsackItem, KnapsackSolution, KnapsackSolver};

pub struct FptasDpSolver;

impl FptasDpSolver {
    fn scale_items(input: &KnapsackInput) -> Vec<KnapsackItem> {
        let max_value = input.items.iter().map(|item| item.value).max().unwrap();
        let scale =
            f64::from(input.granularity) / f64::from(max_value) * (input.items.len() as f64);
        input
            .items
            .iter()
            .map(|item| {
                KnapsackItem::new(
                    item.weight,
                    (f64::from(item.value) * scale).floor().max(1.0) as u32,
                )
            })
            .collect()
    }

    fn get_actual_solution(
        scaled_solution: KnapsackSolution,
        input: &KnapsackInput,
    ) -> KnapsackSolution {
        let max_value = input.items.iter().map(|item| item.value).max().unwrap();
        let scale =
            f64::from(max_value) / (f64::from(input.granularity) * (input.items.len() as f64));
        let total_value = scaled_solution.total_value as f64 * scale;
        KnapsackSolution {
            items: scaled_solution.items,
            total_value: total_value.floor() as u64,
        }
    }
}

impl KnapsackSolver for FptasDpSolver {
    fn solve(input: &KnapsackInput) -> KnapsackSolution {
        let scaled_items = FptasDpSolver::scale_items(input);
        let scaled_input =
            KnapsackInput::new(scaled_items, input.capacity, Some(input.granularity)).unwrap();

        let scaled_solution = DpSolver::solve(&scaled_input);

        FptasDpSolver::get_actual_solution(scaled_solution, input)
    }
}

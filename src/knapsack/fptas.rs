use super::dp::DpSolver;
use super::{KnapsackInput, KnapsackItem, KnapsackMethod, KnapsackSolution, KnapsackSolver};

pub struct FptasDpSolver;

impl FptasDpSolver {
    fn get_scaling_factor(input: &KnapsackInput) -> f64 {
        let max_value = input.items.iter().map(|item| item.value).max().unwrap();
        max_value as f64 / (input.granularity as usize * input.items.len()) as f64
    }

    fn scale_items(input: &KnapsackInput) -> Vec<KnapsackItem> {
        let scale = FptasDpSolver::get_scaling_factor(input);

        input
            .items
            .iter()
            .map(|item| {
                KnapsackItem::new(
                    item.weight,
                    (f64::from(item.value) / scale).floor().max(1.0) as u32,
                )
            })
            .collect()
    }

    fn get_actual_solution(
        scaled_solution: KnapsackSolution,
        input: &KnapsackInput,
    ) -> KnapsackSolution {
        let scale = FptasDpSolver::get_scaling_factor(input);
        let total_value = scaled_solution.total_value as f64 * scale;

        KnapsackSolution {
            items: scaled_solution.items,
            total_value: total_value.floor() as u64,
        }
    }
}

impl KnapsackSolver for FptasDpSolver {
    fn solve(&self, input: &KnapsackInput) -> KnapsackSolution {
        let scaled_items = FptasDpSolver::scale_items(input);
        let scaled_input =
            KnapsackInput::new(scaled_items, input.capacity, input.granularity).unwrap();

        let scaled_solution = DpSolver.solve(&scaled_input);

        FptasDpSolver::get_actual_solution(scaled_solution, input)
    }

    fn method(&self) -> KnapsackMethod {
        KnapsackMethod::Fptas
    }
}

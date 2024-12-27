use super::{KnapsackInput, KnapsackSolution, KnapsackSolver};

pub struct BktSolver;

impl BktSolver {
    fn bkt_non_recursive(input: &KnapsackInput) -> KnapsackSolution {
        let n = input.items.len();
        let mut best_solution = KnapsackSolution {
            items: Vec::new(),
            total_value: 0,
        };
        let mut stack = Vec::new();
        let mut current_solution = KnapsackSolution {
            items: Vec::new(),
            total_value: 0,
        };
        let mut current_weight = 0;
        let mut current_value = 0;
        let mut current_item = 0;

        loop {
            if current_item < n {
                let item = &input.items[current_item];
                if current_weight + item.weight <= input.capacity {
                    stack.push((current_item, current_weight, current_value));
                    current_solution.items.push(current_item);
                    current_solution.total_value += u64::from(item.value);
                    current_weight += item.weight;
                    current_value += item.value;
                    current_item += 1;
                    continue;
                }
            }

            if current_solution.total_value > best_solution.total_value {
                best_solution = current_solution.clone();
            }

            if let Some((item, weight, value)) = stack.pop() {
                current_item = item + 1;
                current_weight = weight;
                current_value = value;
                current_solution.items.pop();
                current_solution.total_value -= u64::from(input.items[item].value);
            } else {
                break;
            }
        }

        best_solution
    }
}

impl KnapsackSolver for BktSolver {
    fn solve(input: &KnapsackInput) -> KnapsackSolution {
        BktSolver::bkt_non_recursive(input)
    }
}

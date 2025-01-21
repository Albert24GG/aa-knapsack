use super::{KnapsackInput, KnapsackMethod, KnapsackSolution, KnapsackSolver};

use ndarray::Array2;

pub struct DpSolver;

impl DpSolver {
    fn gen_table(input: &KnapsackInput) -> Array2<u64> {
        let n = input.items.len();
        let max_profit = input.max_item_profit();
        let max_cost = input.max_cost();
        let items = &input.items;
        let mut dp_table = Array2::<u64>::zeros((n, max_profit as usize + 1));

        dp_table.row_mut(0).fill(u64::MAX - max_cost as u64);
        dp_table[(0, 0)] = 0;
        dp_table[(0, items[0].profit as usize)] = items[0].weight.into();

        for i in 1..n {
            // Copy the previous row elements that have a profit less than the current item's value
            for profit in 1usize..items[i].profit as usize {
                dp_table[(i, profit)] = dp_table[(i - 1, profit)];
            }
            // Try to improve the weight for a given profit using the current item
            for profit in items[i].profit as usize..=max_profit as usize {
                dp_table[(i, profit)] = dp_table[(i - 1, profit)].min(
                    dp_table[(i - 1, profit - items[i].profit as usize)] + items[i].weight as u64,
                );
            }
        }

        dp_table
    }

    fn gen_path(dp_table: &Array2<u64>, input: &KnapsackInput, max_profit: u64) -> Vec<usize> {
        let mut path = Vec::new();
        let mut profit = max_profit;

        for i in (1..input.items.len()).rev() {
            if dp_table[(i, profit as usize)] != dp_table[(i - 1, profit as usize)] {
                path.push(i);
                profit -= u64::from(input.items[i].profit);
            }
            if profit == 0 {
                break;
            }
        }

        if profit > 0 {
            path.push(0);
        }

        path
    }
}

impl KnapsackSolver for DpSolver {
    fn solve(&self, input: &KnapsackInput) -> KnapsackSolution {
        let dp_table = DpSolver::gen_table(input);

        let max_profit = dp_table
            .row(input.items.len() - 1)
            .iter()
            .enumerate()
            .filter(|(_, &weight)| weight <= u64::from(input.capacity))
            .map(|(profit, _)| profit as u64)
            .max()
            .unwrap();

        let selected_items = DpSolver::gen_path(&dp_table, input, max_profit);

        KnapsackSolution {
            items: selected_items,
            total_value: max_profit,
        }
    }

    fn method(&self) -> KnapsackMethod {
        KnapsackMethod::Dp
    }
}

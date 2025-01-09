use super::{KnapsackInput, KnapsackMethod, KnapsackSolution, KnapsackSolver};

pub struct DpSolver;

impl DpSolver {
    fn gen_table(input: &KnapsackInput) -> Vec<u64> {
        let n = input.items.len();
        let max_profit = input.max_items_profit();
        let items = &input.items;
        // We consider f(i, j) as the min weight that can be achieved with a profit of j
        // using the first i items.
        // The dp formula is f(i, j) = min(f(i - 1, j), f(i - 1, j - v[i]) + w[i])
        // Therefore we only need to keep track of the last row of the dp table.
        // Using a 1D array to represent the dp table. This means we will not be able to
        // reconstruct the path.
        // The formula becomes f(j) = min(f(j), f(j - v[i]) + w[i]), considering that
        // we are iterating in reverse order.
        let mut dp_table = vec![u64::MAX - max_profit; max_profit as usize + 1];

        dp_table[0] = 0;
        dp_table[items[0].value as usize] = items[0].weight.into();

        for i in 1..n {
            for profit in (items[i].value.into()..=max_profit).rev() {
                let new_weight =
                    dp_table[(profit - items[i].value as u64) as usize] + items[i].weight as u64;

                dp_table[profit as usize] = dp_table[profit as usize].min(new_weight);
            }
        }

        dp_table
    }
}

impl KnapsackSolver for DpSolver {
    fn solve(&self, input: &KnapsackInput) -> KnapsackSolution {
        let dp_table = DpSolver::gen_table(input);

        let max_profit = dp_table
            .iter()
            .enumerate()
            .filter(|(_, &weight)| weight <= input.capacity.into())
            .map(|(profit, _)| profit)
            .max()
            .unwrap() as u64;

        KnapsackSolution {
            items: None,
            total_value: max_profit,
        }
    }

    fn method(&self) -> KnapsackMethod {
        KnapsackMethod::Dp
    }
}

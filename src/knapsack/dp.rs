use super::{KnapsackInput, KnapsackMethod, KnapsackSolution, KnapsackSolver};

pub struct DpSolver;

impl DpSolver {
    fn gen_table(input: &KnapsackInput) -> Vec<Vec<u64>> {
        let n = input.items.len();
        let max_profit = input.max_item_profit();
        let max_cost = input.max_cost();
        let items = &input.items;
        let mut dp_table = vec![vec![0; max_profit as usize + 1]; n];

        // base case: if target can be achieved by taking the first item
        dp_table[0]
            .iter_mut()
            .take(items[0].value as usize + 1)
            .for_each(|x| *x = items[0].weight.into());

        // base case: if target cannot be achieved by taking the first item
        // use a big value to represent infinity
        // to prevent overflow, subtract by the maximum cost
        dp_table[0]
            .iter_mut()
            .skip(items[0].value as usize + 1)
            .for_each(|x| *x = u64::MAX - u64::from(max_cost));

        dp_table[0][0] = 0;

        for i in 1..n {
            for profit in 1..=max_profit {
                dp_table[i][profit as usize] = if profit >= u64::from(items[i].value) {
                    let prev_profit = profit - u64::from(items[i].value);
                    dp_table[i - 1][profit as usize]
                        .min(dp_table[i - 1][prev_profit as usize] + u64::from(items[i].weight))
                } else {
                    dp_table[i - 1][profit as usize]
                };
            }
        }

        dp_table
    }

    fn gen_path(dp_table: Vec<Vec<u64>>, input: &KnapsackInput, max_profit: u64) -> Vec<usize> {
        let mut path = Vec::new();
        let mut profit = max_profit;

        for i in (1..input.items.len()).rev() {
            if dp_table[i][profit as usize] != dp_table[i - 1][profit as usize] {
                path.push(i);
                profit -= u64::from(input.items[i].value);
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

        let max_profit = (0..dp_table[0].len() as u64)
            .filter(|&profit| {
                dp_table[input.items.len() - 1][profit as usize] <= u64::from(input.capacity)
            })
            .max()
            .unwrap();

        let selected_items = DpSolver::gen_path(dp_table, input, max_profit);

        KnapsackSolution {
            items: selected_items,
            total_value: max_profit,
        }
    }

    fn method(&self) -> KnapsackMethod {
        KnapsackMethod::Dp
    }
}

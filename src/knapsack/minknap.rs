use super::{KnapsackInput, KnapsackItem, KnapsackMethod, KnapsackSolution, KnapsackSolver};
use bitvec::prelude::*;

struct ItemEfficiency {
    index: usize,
    efficiency: f32,
}

/// This function prepares the items for the minknap algorithm.
/// It sorts the items by efficiency, calculated as value/weight.
/// It also filters out items that are too heavy to be included in the knapsack,
/// and implicitly includes all items that have zero weight.
///
/// Returns a tuple containing:
/// - A vector of ItemEfficiency structs, sorted by efficiency
/// - A BitVec representing the decision vector, with true values for items that are included (it is used for including the zero weight items)
/// - The total profit of the included zero weight items
fn prepare_items(input: &KnapsackInput) -> (Vec<ItemEfficiency>, BitVec, u64) {
    let mut decision_vec = bitvec![0; input.items.len()];
    let mut base_profit = 0u64;

    let mut items: Vec<ItemEfficiency> = input
        .items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if item.weight == 0 {
                decision_vec.set(i, true);
                base_profit += item.profit as u64;
                return None;
            }
            if item.weight as u64 <= input.capacity {
                Some(ItemEfficiency {
                    index: i,
                    efficiency: item.profit as f32 / item.weight as f32,
                })
            } else {
                None
            }
        })
        .collect();

    items.sort_by(|a, b| b.efficiency.partial_cmp(&a.efficiency).unwrap());

    (items, decision_vec, base_profit)
}

#[derive(Default)]
struct BreakSolution {
    break_index: usize,
    // The total profit of the items that have been included (integral only)
    total_profit: u64,
    total_weight: u64,
    // The total profit of the items that have been included (including fractional part of the last item = break_index)
    total_linear_profit: u64,
}

impl BreakSolution {
    /// Calculates the break solution for the given input, marking all the selected items in the decision vector.
    /// The break solution is the solution that includes all items up to the break_index-1, and a fraction of the break_index-th item.
    ///
    /// input - the knapsack input
    /// item_efficiencies - the items sorted by efficiency
    /// decision_vec - the decision vector
    ///
    /// The item_efficiencies and decision_vec should be the same as the ones returned by prepare_items.
    fn new(
        input: &KnapsackInput,
        item_efficiencies: &[ItemEfficiency],
        decision_vec: &mut BitSlice,
    ) -> Self {
        let mut total_profit = 0u64;
        let mut total_weight = 0u64;

        let mut i = 0usize;
        let mut result = BreakSolution::default();

        while i < item_efficiencies.len() {
            let item = &input.items[item_efficiencies[i].index];
            if total_weight + item.weight as u64 <= input.capacity {
                total_weight += item.weight as u64;
                total_profit += item.profit as u64;
                decision_vec.set(item_efficiencies[i].index, true);
            } else {
                let remaining_weight = input.capacity - total_weight;
                let break_item_efficiency = item_efficiencies[i].efficiency;
                let total_linear_profit = (total_profit as f32
                    + break_item_efficiency * remaining_weight as f32)
                    .ceil() as u64;

                result = BreakSolution {
                    break_index: i,
                    total_profit,
                    total_weight,
                    total_linear_profit,
                };
                break;
            }

            i += 1;
        }

        // Handle the case when all items can be included
        // In this case, there is no break item
        if i == item_efficiencies.len() {
            result = BreakSolution {
                break_index: i,
                total_profit,
                total_weight,
                total_linear_profit: total_profit,
            };
        }

        result
    }
}

struct MinKnapInstance<'a> {
    /// The weight of the best solution found so far
    best_sol_weight: u64,
    /// The decision vector of the best solution found so far
    best_sol_decisions: BitVec,
    /// A bit vector representing the items included in the best solution
    decision_vec: BitVec,
    /// The efficiencies of the considered items sorted in decreasing order (excluding zero weight items and items that are too heavy)
    item_efficiencies: Vec<ItemEfficiency>,
    /// The total profit of the implicitly included zero weight items
    base_profit: u64,
    /// The break solution
    break_solution: BreakSolution,
    problem_instance: &'a KnapsackInput,
    /// The lower bound of the core problem
    s: usize,
    /// The upper bound of the core problem
    t: usize,
    /// The best feasible profit found so far
    profit_lower_bound: u64,
    /// The max weight a state can reach to still be feasible (detailed in the paper & book)
    max_allowed_weight: u64,
}

#[derive(Clone, Default)]
struct MinKnapState {
    weight: u64,
    profit: u64,
    decisions: BitVec,
}

impl<'a> MinKnapInstance<'a> {
    fn new(input: &'a KnapsackInput) -> Self {
        let (item_efficiencies, mut decision_vec, base_profit) = prepare_items(input);
        let break_solution =
            BreakSolution::new(input, &item_efficiencies, decision_vec.as_mut_bitslice());
        let max_allowed_weight = input.capacity + break_solution.total_weight;

        // Initially, our best solution is the break solution
        // From the book "Knapsack Problems" (p137), for the Primal-Dual DP Algorithm, the observation is that, generally, only a few items around the break
        // index need to be included/excluded to get the optimal solution.
        //
        // Thus, let z_s,t(d) for s = 0..b, t = b-1..n, d = 0..2*capacity, be the optimal solution
        // to the problem:
        //
        // z_s,t(d) = sum_{j=0}^{s-1} p_j + max{ sum_{j=s}^{t} p_j*x_j | sum_{j=s}^{t} w_j*x_j <= d - sum_{j=0}^{s-1} w_j, x_j in {0,1}, j = s..t }
        //
        // So z_s,t(d) is an optimal solution defined on items j=s..t, where items j<s have been
        // fixed to 1, and items j>t have been fixed to 0.
        let b = break_solution.break_index;
        let s = b;
        let t = b - 1;

        let profit_lower_bound = break_solution.total_profit;
        let best_sol_weight = break_solution.total_weight;

        MinKnapInstance {
            best_sol_weight,
            best_sol_decisions: BitVec::EMPTY,
            decision_vec,
            item_efficiencies,
            base_profit,
            break_solution,
            problem_instance: input,
            s,
            t,
            profit_lower_bound,
            max_allowed_weight,
        }
    }

    /// Returns the item at the given efficiency order index
    fn get_item(&self, efficiency_order_idx: usize) -> KnapsackItem {
        self.problem_instance.items[self.item_efficiencies[efficiency_order_idx].index]
    }

    /// Returns the upper bound of the core problem with the given current bounds [s, t] and state
    fn get_profit_upper_bound(&self, current_state: &MinKnapState, s: usize, t: usize) -> u64 {
        if current_state.weight <= self.problem_instance.capacity {
            // Under capacity, we can try expanding the core by including the next item after t
            if t + 1 >= self.problem_instance.items.len() - 1 {
                // If we are already at the last item, we can't expand the core anymore
                current_state.profit
            } else {
                // Try linearly expanding the core
                let weight_diff = self.problem_instance.capacity - current_state.weight;
                let next_item_efficiency = self.item_efficiencies[t + 1].efficiency;

                current_state.profit + (weight_diff as f32 * next_item_efficiency).ceil() as u64
            }
        } else {
            // Over capacity, we can try reducing the core by excluding the next item after s

            if s == 0 {
                // If we are already at the first item, we can't reduce the core anymore
                current_state.profit
            } else {
                // Try linearly reducing the core
                let weight_diff = current_state.weight - self.problem_instance.capacity;
                let prev_item_efficiency = self.item_efficiencies[s - 1].efficiency;

                current_state
                    .profit
                    .saturating_sub((weight_diff as f32 * prev_item_efficiency).ceil() as u64)
            }
        }
    }

    /// Try updating the best profit found so far with the given state
    /// This should be called only on newly found feasible states
    fn try_update_lower_bound(&mut self, state: &MinKnapState) {
        if state.weight <= self.problem_instance.capacity && state.profit > self.profit_lower_bound
        {
            self.profit_lower_bound = state.profit;
            self.best_sol_weight = state.weight;

            self.best_sol_decisions = state.decisions.clone();
        }
    }

    /// Mark the decision of the given efficiency order index in the state
    ///
    /// include - whether to include the item or not
    fn mark_decision(&self, state: &mut MinKnapState, efficiency_order_idx: usize, include: bool) {
        state
            .decisions
            .set(self.item_efficiencies[efficiency_order_idx].index, include);
    }

    /// Explore the core problem by trying to include item t
    /// This should be called after expanding the core (incrementing t)
    fn explore_item_t(
        &mut self,
        current_states: &mut [MinKnapState],
        next_states: &mut Vec<MinKnapState>,
    ) {
        // We explore both the possibility of including or not including the item t for each state
        // in current_states
        // When building next_states, we also make sure to keep it ordered both by profit and weight
        // so that we can easily discard duplicates and dominated states
        //
        // So, for each state in next_states: profit_i <= profit_{i+1} and weight_i <= weight_{i+1}
        //
        // A dominated state is a state that will have a higher weight and a lower profit than the
        // current next_state (the last state in next_states)
        //
        // This ordering is achieved by using two iterators: one for trying to insert the item in the
        // states and one for leaving the states unchanged

        let item = self.get_item(self.t);
        let state_count = current_states.len();

        // The index of the state in current_states that we are trying to insert the item in
        let mut insert_index = 0usize;
        // The index of the state in current_states that we are leaving unchanged
        let mut no_insert_index = 0usize;

        while insert_index < state_count || no_insert_index < state_count {
            // This check help us maintain the ordering of the states by weight
            // The profit ordering is maintained by the fact that we are discarding dominated states
            if no_insert_index >= state_count
                || current_states[no_insert_index].weight
                    > current_states[insert_index].weight + item.weight as u64
            {
                // The new state that we would get by including the item in the current state
                let mut new_state = MinKnapState {
                    weight: current_states[insert_index].weight + item.weight as u64,
                    profit: current_states[insert_index].profit + item.profit as u64,
                    decisions: BitVec::EMPTY,
                };

                if new_state.weight > self.max_allowed_weight {
                    // If the weight of the state is too high, we discard it
                    insert_index += 1;
                    continue;
                }

                if !next_states.is_empty() && next_states.last().unwrap().profit >= new_state.profit
                {
                    // If the profit of the state is not higher than the last state in next_states,
                    // then it is dominated and we discard it
                    insert_index += 1;
                    continue;
                }

                if self.get_profit_upper_bound(&new_state, self.s, self.t)
                    <= self.profit_lower_bound
                {
                    // If the upper bound of the state is not higher than the best feasible profit
                    // found so far, then we discard it
                    insert_index += 1;
                    continue;
                }

                // If we know that the state will no longer be visited, we can move it out
                new_state.decisions = if insert_index < no_insert_index {
                    std::mem::take(&mut current_states[insert_index].decisions)
                } else {
                    current_states[insert_index].decisions.clone()
                };

                // This state is feasible, so we mark it in the decision vector
                self.mark_decision(&mut new_state, self.t, true);

                // Only changed states(new states) can create new lower bounds
                self.try_update_lower_bound(&new_state);

                // If this state dominates the last state in next_states, overwrite it,
                // otherwise, add it to the end of next_states
                next_states
                    .last_mut()
                    .filter(|last_state| last_state.weight == new_state.weight)
                    .map(|last_state| std::mem::swap(last_state, &mut new_state))
                    .or_else(|| {
                        next_states.push(new_state);
                        Some(())
                    });

                insert_index += 1;
            } else {
                let current_state = &current_states[no_insert_index];

                if !next_states.is_empty()
                    && next_states.last().unwrap().profit >= current_state.profit
                {
                    // If the profit of the state is not higher than the last state in next_states,
                    // then it is dominated and we discard it
                    no_insert_index += 1;
                    continue;
                }

                if self.get_profit_upper_bound(current_state, self.s, self.t)
                    <= self.profit_lower_bound
                {
                    // If the upper bound of the state is not higher than the best feasible profit
                    // found so far, then we discard it
                    no_insert_index += 1;
                    continue;
                }

                // If we know that the state will no longer be visited, we can move it out
                let mut current_state = if no_insert_index < insert_index {
                    std::mem::take(&mut current_states[no_insert_index])
                } else {
                    current_states[no_insert_index].clone()
                };

                // If this state dominates the last state in next_states, overwrite it,
                // otherwise, add it to the end of next_states
                next_states
                    .last_mut()
                    .filter(|last_state| last_state.weight == current_state.weight)
                    .map(|last_state| std::mem::swap(last_state, &mut current_state))
                    .or_else(|| {
                        next_states.push(current_state);
                        Some(())
                    });

                no_insert_index += 1;
            }
        }
    }

    /// Explore the core problem by trying to exclude item s
    /// This should be called after expanding the core (decrementing s)
    fn explore_item_s(
        &mut self,
        current_states: &mut [MinKnapState],
        next_states: &mut Vec<MinKnapState>,
    ) {
        // For more details, see the comments in explore_item_t
        // The only difference is that we are excluding the item s instead of including the item t

        let item = self.get_item(self.s);
        let state_count = current_states.len();

        // The index of the state in current_states that we are trying to remove the item from
        let mut remove_index = 0usize;
        // The index of the state in current_states that we are leaving unchanged
        let mut no_remove_index = 0usize;

        while remove_index < state_count || no_remove_index < state_count {
            // This check help us maintain the ordering of the states by weight
            // The profit ordering is maintained by the fact that we are discarding dominated states
            if remove_index >= state_count
                || current_states[no_remove_index].weight
                    <= current_states[remove_index].weight - item.weight as u64
            {
                let current_state = &current_states[no_remove_index];

                if !next_states.is_empty()
                    && next_states.last().unwrap().profit >= current_state.profit
                {
                    // If the profit of the state is not higher than the last state in next_states,
                    // then it is dominated and we discard it
                    no_remove_index += 1;
                    continue;
                }

                if self.get_profit_upper_bound(current_state, self.s, self.t)
                    <= self.profit_lower_bound
                {
                    // If the upper bound of the state is not higher than the best feasible profit
                    // found so far, then we discard it
                    no_remove_index += 1;
                    continue;
                }

                // let mut current_state = current_state.clone();
                // If we know that the state will no longer be visited, we can move it out
                let mut current_state = if no_remove_index < remove_index {
                    std::mem::take(&mut current_states[no_remove_index])
                } else {
                    current_states[no_remove_index].clone()
                };

                // If this state dominates the last state in next_states, overwrite it,
                // otherwise, add it to the end of next_states
                next_states
                    .last_mut()
                    .filter(|last_state| last_state.weight == current_state.weight)
                    .map(|last_state| std::mem::swap(last_state, &mut current_state))
                    .or_else(|| {
                        next_states.push(current_state);
                        Some(())
                    });

                no_remove_index += 1;
            } else {
                // The new state that we would get by including the item in the current state
                let mut new_state = MinKnapState {
                    weight: current_states[remove_index].weight - item.weight as u64,
                    profit: current_states[remove_index].profit - item.profit as u64,
                    decisions: BitVec::EMPTY,
                };

                if new_state.weight > self.max_allowed_weight {
                    // If the weight of the state is too high, we discard it
                    remove_index += 1;
                    continue;
                }

                if !next_states.is_empty() && next_states.last().unwrap().profit >= new_state.profit
                {
                    // If the profit of the state is not higher than the last state in next_states,
                    // then it is dominated and we discard it
                    remove_index += 1;
                    continue;
                }

                if self.get_profit_upper_bound(&new_state, self.s, self.t)
                    <= self.profit_lower_bound
                {
                    // If the upper bound of the state is not higher than the best feasible profit
                    // found so far, then we discard it
                    remove_index += 1;
                    continue;
                }

                // If we know that the state will no longer be visited, we can move it out
                new_state.decisions = if remove_index < no_remove_index {
                    std::mem::take(&mut current_states[remove_index].decisions)
                } else {
                    current_states[remove_index].decisions.clone()
                };

                // This state is feasible, so we mark it in the decision vector
                self.mark_decision(&mut new_state, self.s, false);

                // Only changed states(new states) can create new lower bounds
                self.try_update_lower_bound(&new_state);

                // If this state dominates the last state in next_states, overwrite it,
                // otherwise, add it to the end of next_states
                next_states
                    .last_mut()
                    .filter(|last_state| last_state.weight == new_state.weight)
                    .map(|last_state| std::mem::swap(last_state, &mut new_state))
                    .or_else(|| {
                        next_states.push(new_state);
                        Some(())
                    });

                remove_index += 1;
            }
        }
    }

    /// Swap the state buffers
    fn swap_state_buffers(
        &mut self,
        current_states: &mut Vec<MinKnapState>,
        next_states: &mut Vec<MinKnapState>,
    ) {
        current_states.clear();
        std::mem::swap(current_states, next_states);
    }

    /// Solve the problem, returning the best profit found and its corresponding weight
    fn solve(mut self) -> (u64, u64) {
        // Check the edge case when the break solution is already the best solution
        if self.break_solution.break_index == self.problem_instance.items.len() {
            return (
                self.break_solution.total_profit,
                self.break_solution.total_weight,
            );
        }

        let mut current_states = Vec::<MinKnapState>::new();
        let mut next_states = Vec::<MinKnapState>::new();

        let mut visited_items_count = 0usize;
        let n = self.problem_instance.items.len();

        current_states.push(MinKnapState {
            weight: self.break_solution.total_weight,
            profit: self.break_solution.total_profit,
            decisions: {
                let mut d = self.decision_vec.clone();
                d.resize(n, false);
                d
            },
        });

        while !current_states.is_empty() && visited_items_count < n {
            if self.t + 1 < n {
                self.t += 1;
                self.explore_item_t(&mut current_states, &mut next_states);
                self.swap_state_buffers(&mut current_states, &mut next_states);

                if self.best_sol_weight == self.problem_instance.capacity {
                    // If we have already found the best solution, we can stop
                    break;
                }
                visited_items_count += 1;
            }

            if self.s > 0 {
                self.s -= 1;
                self.explore_item_s(&mut current_states, &mut next_states);
                self.swap_state_buffers(&mut current_states, &mut next_states);

                if self.best_sol_weight == self.problem_instance.capacity {
                    // If we have already found the best solution, we can stop
                    break;
                }
                visited_items_count += 1;
            }
        }

        println!("Best solution decision vector:{:?}", {
            if self.best_sol_decisions.is_empty() {
                self.decision_vec
            } else {
                self.best_sol_decisions
            }
        });

        // TODO: build the decision vector
        (
            self.profit_lower_bound + self.base_profit,
            self.best_sol_weight,
        )
    }
}

pub struct MinKnapSolver;

impl KnapsackSolver for MinKnapSolver {
    fn solve(&self, input: &KnapsackInput) -> KnapsackSolution {
        let instance = MinKnapInstance::new(input);
        let (profit, weight) = instance.solve();

        println!("Profit: {}, Weight: {}", profit, weight);

        KnapsackSolution {
            items: vec![],
            total_value: profit,
        }
    }

    fn method(&self) -> KnapsackMethod {
        KnapsackMethod::MinKnap
    }
}

pub mod bkt;
pub mod dp;
pub mod fptas;
pub mod minknap;
mod sol_tree;

use std::io::BufRead;

use serde::Serialize;
use strum_macros::{AsRefStr, IntoStaticStr};
use thiserror::Error;

pub trait KnapsackSolver: Sync {
    fn solve(&self, input: &KnapsackInput) -> KnapsackSolution;

    fn method(&self) -> KnapsackMethod;
}

#[derive(Clone, Copy, Debug)]
pub struct KnapsackItem {
    weight: u64,
    profit: u64,
}

#[derive(Debug, Clone)]
pub struct KnapsackInput {
    items: Vec<KnapsackItem>,
    capacity: u64,
    granularity: u32,
}

impl KnapsackItem {
    pub fn new(weight: u64, value: u64) -> Self {
        KnapsackItem {
            weight,
            profit: value,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct KnapsackSolution {
    // index of items selected
    pub items: Vec<usize>,
    // total value/profit of items selected
    pub total_value: u64,
}

#[derive(Error, Debug)]
pub enum KnapsackInputError {
    #[error("Invalid item count")]
    InvalidItemCount,
    #[error("Missing item count")]
    MissingItemCount,
    #[error("Invalid capacity")]
    InvalidCapacity,
    #[error("Missing capacity")]
    MissingCapacity,
    #[error("Invalid granularity")]
    InvalidGranularity,
    #[error("Invalid item weight")]
    InvalidItemWeight,
    #[error("Invalid item value")]
    InvalidItemValue,
    #[error("Invalid item specification")]
    InvalidItemSpecification,
    #[error("Failed to read input")]
    ReadError(#[from] std::io::Error),
    #[error("Insufficient items provided")]
    InsufficientItems,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AsRefStr, IntoStaticStr)]
pub enum KnapsackMethod {
    Dp,
    Bkt,
    Fptas,
    MinKnap,
}

impl KnapsackInput {
    /// Parse the input for the knapsack problem
    ///
    /// The input format is as follows:
    /// n - number of items on the first line
    /// capacity - the capacity of the knapsack on the second line
    /// n lines with two integers each, representing the value and weight of each item
    pub fn parse_input(input: impl BufRead) -> Result<KnapsackInput, KnapsackInputError> {
        // Parse only the non-empty lines
        let mut lines = input
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| (!line.is_empty()).then_some(line));

        let n = lines
            .next()
            .ok_or(KnapsackInputError::MissingItemCount)?
            .parse()
            .map_err(|_| KnapsackInputError::InvalidItemCount)?;

        let capacity = lines
            .next()
            .ok_or(KnapsackInputError::MissingCapacity)?
            .parse()
            .map_err(|_| KnapsackInputError::InvalidCapacity)?;

        let mut items = Vec::with_capacity(n);
        for line in lines.take(n) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(KnapsackInputError::InvalidItemSpecification);
            }

            let value: u64 = parts[0]
                .parse()
                .map_err(|_| KnapsackInputError::InvalidItemValue)?;
            let weight: u64 = parts[1]
                .parse()
                .map_err(|_| KnapsackInputError::InvalidItemWeight)?;

            items.push(KnapsackItem::new(weight, value));
        }

        if items.len() < n {
            return Err(KnapsackInputError::InsufficientItems);
        }

        KnapsackInput::new(items, capacity, 1)
    }

    fn validate_capacity(capacity: u64) -> Result<(), KnapsackInputError> {
        if capacity == 0 {
            return Err(KnapsackInputError::InvalidCapacity);
        }
        Ok(())
    }

    fn validate_granularity(granularity: u32) -> Result<(), KnapsackInputError> {
        if granularity == 0 {
            return Err(KnapsackInputError::InvalidGranularity);
        }
        Ok(())
    }

    pub fn new(
        items: Vec<KnapsackItem>,
        capacity: u64,
        granularity: u32,
    ) -> Result<Self, KnapsackInputError> {
        Self::validate_capacity(capacity)?;
        Self::validate_granularity(granularity)?;

        Ok(KnapsackInput {
            items,
            capacity,
            granularity,
        })
    }

    pub fn set_granularity(&mut self, granularity: u32) -> Result<(), KnapsackInputError> {
        if granularity == 0 {
            return Err(KnapsackInputError::InvalidGranularity);
        }
        self.granularity = granularity;
        Ok(())
    }

    pub fn max_cost(&self) -> u64 {
        self.items.iter().map(|item| item.weight).max().unwrap()
    }

    pub fn max_item_profit(&self) -> u64 {
        self.items.iter().map(|item| u64::from(item.profit)).sum()
    }
}

pub mod bkt;
pub mod dp;

use thiserror::Error;

pub trait KnapsackSolver {
    fn solve(input: &KnapsackInput) -> KnapsackSolution;
}

#[derive(Clone, Copy, Debug)]
pub struct KnapsackItem {
    weight: u32,
    value: u32,
}

#[derive(Debug, Clone)]
pub struct KnapsackInput {
    items: Vec<KnapsackItem>,
    capacity: u32,
    granularity: u32,
}

impl KnapsackItem {
    pub fn new(weight: u32, value: u32) -> Self {
        KnapsackItem { weight, value }
    }
}

#[derive(Debug, Clone)]
pub struct KnapsackSolution {
    // index of items selected
    pub items: Vec<usize>,
    // total value/profit of items selected
    pub total_value: u64,
}

#[derive(Error, Debug)]
pub enum KnapsackInputError {
    #[error("Invalid capacity")]
    InvalidCapacity,
    #[error("Invalid granularity")]
    InvalidGranularity,
    #[error("Invalid item weight")]
    InvalidItemWeight,
    #[error("Invalid item value")]
    InvalidItemValue,
}

impl KnapsackInput {
    fn validate_items(items: &[KnapsackItem]) -> Result<(), KnapsackInputError> {
        if items.iter().any(|item| item.weight == 0) {
            return Err(KnapsackInputError::InvalidItemWeight);
        }
        if items.iter().any(|item| item.value == 0) {
            return Err(KnapsackInputError::InvalidItemValue);
        }
        Ok(())
    }

    fn validate_capacity(capacity: u32) -> Result<(), KnapsackInputError> {
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
        capacity: u32,
        granularity: Option<u32>,
    ) -> Result<Self, KnapsackInputError> {
        Self::validate_items(&items)?;
        Self::validate_capacity(capacity)?;
        granularity.map_or(Ok(()), Self::validate_granularity)?;

        Ok(KnapsackInput {
            items,
            capacity,
            granularity: granularity.unwrap_or(1),
        })
    }

    pub fn set_granularity(&mut self, granularity: u32) -> Result<(), KnapsackInputError> {
        if granularity == 0 {
            return Err(KnapsackInputError::InvalidGranularity);
        }
        self.granularity = granularity;
        Ok(())
    }

    pub fn max_cost(&self) -> u32 {
        self.items.iter().map(|item| item.weight).max().unwrap()
    }

    pub fn max_item_profit(&self) -> u64 {
        self.items.iter().map(|item| u64::from(item.value)).sum()
    }
}

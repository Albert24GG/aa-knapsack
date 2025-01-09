pub mod bkt;
pub mod dp;
pub mod fptas;

use serde::Serialize;
use strum_macros::{AsRefStr, IntoStaticStr};
use thiserror::Error;

pub trait KnapsackSolver: Sync {
    fn solve(&self, input: &KnapsackInput) -> KnapsackSolution;

    fn method(&self) -> KnapsackMethod;
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

#[derive(Debug, Clone, Serialize)]
pub struct KnapsackSolution {
    // index of items selected
    pub items: Option<Vec<usize>>,
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AsRefStr, IntoStaticStr)]
pub enum KnapsackMethod {
    Dp,
    Bkt,
    Fptas,
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
        granularity: u32,
    ) -> Result<Self, KnapsackInputError> {
        Self::validate_items(&items)?;
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

    pub fn max_cost(&self) -> u32 {
        self.items.iter().map(|item| item.weight).max().unwrap()
    }

    pub fn max_items_profit(&self) -> u64 {
        self.items.iter().map(|item| u64::from(item.value)).sum()
    }
}

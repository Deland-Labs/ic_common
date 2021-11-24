use std::collections::HashSet;

use candid::{CandidType, Deserialize, Principal};

use crate::constants::{
    PAGE_INPUT_MAX_LIMIT, PAGE_INPUT_MAX_OFFSET, PAGE_INPUT_MIN_LIMIT, PAGE_INPUT_MIN_OFFSET,
};
use crate::errors::{ServiceError, ServiceResult};

#[cfg(test)]
mod tests;

#[derive(CandidType, Deserialize)]
pub struct GetPageInput {
    pub offset: usize,
    pub limit: usize,
}

impl GetPageInput {
    pub fn validate(&self) -> ServiceResult<()> {
        let max_offset = PAGE_INPUT_MAX_OFFSET;
        let min_offset = PAGE_INPUT_MIN_OFFSET;
        if self.offset > max_offset || self.offset < min_offset {
            return Err(ServiceError::ValueShouldBeInRangeError {
                field: "offset".to_string(),
                min: min_offset,
                max: max_offset,
            });
        }
        let max_limit = PAGE_INPUT_MAX_LIMIT;
        let min_limit = PAGE_INPUT_MIN_LIMIT;
        if self.limit > max_limit || self.limit < min_limit {
            return Err(ServiceError::ValueShouldBeInRangeError {
                field: "limit".to_string(),
                min: min_limit,
                max: max_limit,
            });
        }
        Ok(())
    }
}

#[derive(CandidType, Deserialize)]
pub struct GetPageOutput<T> {
    pub items: Vec<T>,
}

impl<T> GetPageOutput<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}


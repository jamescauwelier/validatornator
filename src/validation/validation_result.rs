use crate::validation::validation_error::ValidationError;

pub type ValidationResult<T> = Result<T, ValidationError>;
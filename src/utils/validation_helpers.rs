use crate::prelude::{ValidationError, ValidationResult};

pub trait ValidationStarter<T> {
    fn must(self) -> ValidationResult<T>;
}

impl<T> ValidationStarter<T> for T {
    fn must(self) -> ValidationResult<T> {
        Ok(self)
    }
}

pub trait ValidationHelper<T> {
    fn validate(self, condition: Box<dyn FnOnce(&T) -> bool>, error_message: &str) -> ValidationResult<T>;
}

impl<T> ValidationHelper<T> for ValidationResult<T> {
    fn validate(self, condition: Box<dyn FnOnce(&T) -> bool>, error_message: &str) -> ValidationResult<T> {
        match self {
            Ok(value) => {
                if condition(&value) {
                    Ok(value)
                } else {
                    Err(ValidationError::new(error_message))
                }
            },
            error => error
        }
    }
}
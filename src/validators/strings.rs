use crate::utils::validation_error::ValidationError;
use crate::utils::validation_result::ValidationResult;

pub trait StringValidator {
    fn be_not_empty(self) -> ValidationResult<String>;
    fn be_longer_than(self, length: usize) -> ValidationResult<String>;
    fn be_shorter_than(self, length: usize) -> ValidationResult<String>;
}

impl StringValidator for Result<String, ValidationError> {
    fn be_not_empty(self) -> ValidationResult<String> {
        match self {
            Ok(value) => {
                if value.is_empty() {
                    Err(ValidationError::new("The string cannot be empty."))
                } else {
                    Ok(value)
                }
            },
            error => error
        }
    }

    fn be_longer_than(self, length: usize) -> ValidationResult<String> {
        match self {
            Ok(value) => {
                if value.len() > length {
                    Ok(value)
                } else {
                    Err(ValidationError::new(format!("The string must be longer than {} characters, but was {} instead.", length, value.len()).as_str()))
                }
            },
            error => error
        }
    }

    fn be_shorter_than(self, length: usize) -> ValidationResult<String> {
        match self {
            Ok(value) => {
                if value.len() < length {
                    Ok(value)
                } else {
                    Err(ValidationError::new(format!("The string must be shorter than {} characters, but was {} instead.", length, value.len()).as_str()))
                }
            },
            error => error
        }
    }
}
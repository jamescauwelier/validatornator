use crate::utils::validation_error::ValidationError;

pub type ValidationResult<T> = Result<T, ValidationError>;

pub(crate) trait ValidationResultValue<T> {
    fn get(&self) -> Option<&T>;
}

impl<T> ValidationResultValue<T> for ValidationResult<T> {
    fn get(&self) -> Option<&T> {
        match self {
            Ok(value) => Some(value),
            Err(_) => panic!("Cannot get the value of an error result.")
        }
    }
}
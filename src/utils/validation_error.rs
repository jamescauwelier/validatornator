use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[error("A utils error occurred: {0}")]
pub struct ValidationError(String);

impl ValidationError {
    pub fn new(msg: &str) -> Self {
        ValidationError(msg.to_string())
    }
}
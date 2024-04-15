use crate::prelude::{ValidationHelper, ValidationResult};

pub trait USizeValidator {
    fn be_larger_than(self, limit: usize) -> ValidationResult<usize>;
    fn be_smaller_than(self, limit: usize) -> ValidationResult<usize>;
}

impl USizeValidator for ValidationResult<usize> {
    fn be_larger_than(self, limit: usize) -> ValidationResult<usize> {
        self.validate(
            Box::new(
                move |v| v > &limit
            ),
            &format! ("Value is not larger than {}.", limit)
        )
    }

    fn be_smaller_than(self, limit: usize) -> ValidationResult<usize> {
        self.validate(
            Box::new(
                move |v| v < &limit
            ),
            &format! ("Value is not smaller than {}.", limit)
        )
    }
}
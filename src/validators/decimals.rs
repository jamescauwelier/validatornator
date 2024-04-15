use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use crate::prelude::*;

pub trait DecimalValidator {
    fn be_larger_than(self, limit: Decimal) -> ValidationResult<Decimal>;
    fn be_smaller_than(self, limit: Decimal) -> ValidationResult<Decimal>;
    fn be_positive(self) -> ValidationResult<Decimal>;
    fn be_negative(self) -> ValidationResult<Decimal>;
    fn be_of_scale(self, precision: u32) -> ValidationResult<Decimal>;
}

impl DecimalValidator for ValidationResult<Decimal> {
    fn be_larger_than(self, limit: Decimal) -> ValidationResult<Decimal> {
        self.validate(
            Box::new(
                move |v| v - limit > Decimal::zero()
            ),
            &format! ("Value is not larger than {}.", limit)
        )
    }

    fn be_smaller_than(self, limit: Decimal) -> ValidationResult<Decimal> {
        self.validate(
            Box::new(
                move |v| (v - limit).is_sign_negative()
            ),
            &format! ("Value is not smaller than {limit}.")
        )
    }

    fn be_positive(self) -> ValidationResult<Decimal> {
        self.validate(
            Box::new(
                move |v| v.is_sign_positive()
            ),
            "Value is not positive."
        )
    }

    fn be_negative(self) -> ValidationResult<Decimal> {
        self.validate(
            Box::new(
                move |v| v.is_sign_negative()
            ),
            "Value is not negative."
        )
    }

    fn be_of_scale(self, precision: u32) -> ValidationResult<Decimal> {
        self.validate(
            Box::new(
                move |v| v.scale() == precision
            ),
            &format! ("Value is not of precision {}.", precision)
        )
    }
}
use rust_decimal_macros::dec;
use validatornator::prelude::*;

#[test]
fn test_is_more_than(){
    assert!(
        dec!(10.01).must().be_larger_than(dec!(10.0)).is_ok()
    )
}

#[test]
fn test_is_less_than(){
    assert!(
        dec!(10.0).must().be_smaller_than(dec!(10.01)).is_ok()
    )
}

#[test]
fn test_is_of_precision(){
    assert!(dec!(10).must().be_of_scale(0).is_ok());
    assert!(dec!(10.0).must().be_of_scale(1).is_ok());
    assert!(dec!(10.00).must().be_of_scale(2).is_ok());
}

#[test]
fn test_is_negative(){
    assert!(dec!(-10.0).must().be_negative().is_ok());
    assert!(dec!(10.0).must().be_negative().is_err());
}

#[test]
fn test_is_positive(){
    assert!(dec!(-10.0).must().be_positive().is_err());
    assert!(dec!(10.0).must().be_positive().is_ok());
}
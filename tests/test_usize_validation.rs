use validatornator::prelude::*;

#[test]
fn test_usize_is_larger_than_limit(){
    assert!(1usize.must().be_larger_than(0).is_ok());
    assert!(1usize.must().be_larger_than(1).is_err());
}

#[test]
fn test_usize_is_smaller_than_limit(){
    assert!(1usize.must().be_smaller_than(2).is_ok());
    assert!(1usize.must().be_smaller_than(1).is_err());
}
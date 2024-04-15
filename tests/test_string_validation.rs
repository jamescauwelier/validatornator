use validatornator::prelude::*;

#[test]
fn test_is_not_empty() {
    let result = String::new().must().be_not_empty();
    assert!(result.is_err());

    let result = String::from("Hello, world!").must().be_not_empty();
    assert!(result.is_ok());

    let result = String::from("abc").must().be_not_empty();
    assert!(result.is_ok());
}

#[test]
fn test_longer_than() {
    let result = String::from("Hello, world!").must().be_longer_than(5);
    assert!(result.is_ok());

    let result = String::from("Hello, world!").must().be_longer_than(15);
    assert!(result.is_err());
}

#[test]
fn test_shorter_than() {
    let result = String::from("Hello, world!").must().be_shorter_than(15);
    assert!(result.is_ok());

    let result = String::from("Hello, world!").must().be_shorter_than(5);
    assert!(result.is_err());
}

#[test]
fn test_combos(){
    let result = String::from("Hello, world!")
        .must()
        .be_not_empty()
        .be_longer_than(5)
        .be_shorter_than(15);
    assert! (result.is_ok());
}
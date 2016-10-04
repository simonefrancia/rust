extern crate forth;

use forth::{Error, Forth, Value};

#[test]
fn no_input_no_stack() {
    assert_eq!(Vec::<Value>::new(), Forth::new().stack());
}

#[test]
#[ignore]
fn numbers_just_get_pushed_onto_the_stack() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 3 4 5").is_ok());
    assert_eq!(vec![1, 2, 3, 4, 5], f.stack());
}

#[test]
#[ignore]
fn non_word_characters_are_separators() {
    let mut f = Forth::new();
    assert!(f.eval("1\u{0000}2\u{0001}3\n4\r5 6\t7").is_ok());
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], f.stack());
}

#[test]
#[ignore]
fn can_add_two_numbers() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 +").is_ok());
    assert_eq!(vec![3], f.stack());
}

#[test]
#[ignore]
fn addition_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("1 +"));
    assert_eq!(Err(Error::StackUnderflow), f.eval("+"));
}

#[test]
#[ignore]
fn can_subtract_two_numbers() {
    let mut f = Forth::new();
    assert!(f.eval("3 4 -").is_ok());
    assert_eq!(vec![-1], f.stack());
}

#[test]
#[ignore]
fn subtraction_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("1 -"));
    assert_eq!(Err(Error::StackUnderflow), f.eval("-"));
}

#[test]
#[ignore]
fn can_multiply_two_numbers() {
    let mut f = Forth::new();
    assert!(f.eval("2 4 *").is_ok());
    assert_eq!(vec![8], f.stack());
}

#[test]
#[ignore]
fn multiplication_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("1 *"));
    assert_eq!(Err(Error::StackUnderflow), f.eval("*"));
}

#[test]
#[ignore]
fn can_divide_two_numbers() {
    let mut f = Forth::new();
    assert!(f.eval("12 3 /").is_ok());
    assert_eq!(vec![4], f.stack());
}

#[test]
#[ignore]
fn performs_integer_division() {
    let mut f = Forth::new();
    assert!(f.eval("8 3 /").is_ok());
    assert_eq!(vec![2], f.stack());
}

#[test]
#[ignore]
fn division_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("1 /"));
    assert_eq!(Err(Error::StackUnderflow), f.eval("/"));
}

#[test]
#[ignore]
fn errors_if_dividing_by_zero() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::DivisionByZero), f.eval("4 0 /"));
}

#[test]
#[ignore]
fn addition_and_subtraction() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 + 4 -").is_ok());
    assert_eq!(vec![-1], f.stack());
}

#[test]
#[ignore]
fn multiplication_and_division() {
    let mut f = Forth::new();
    assert!(f.eval("2 4 * 3 /").is_ok());
    assert_eq!(vec![2], f.stack());
}

#[test]
#[ignore]
fn dup() {
    let mut f = Forth::new();
    assert!(f.eval("1 DUP").is_ok());
    assert_eq!(vec![1, 1], f.stack());
}

#[test]
#[ignore]
fn dup_case_insensitive() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 Dup").is_ok());
    assert_eq!(vec![1, 2, 2], f.stack());
}

#[test]
#[ignore]
fn dup_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("dup"));
}

#[test]
#[ignore]
fn drop() {
    let mut f = Forth::new();
    assert!(f.eval("1 drop").is_ok());
    assert_eq!(Vec::<Value>::new(), f.stack());
}

#[test]
#[ignore]
fn drop_with_two() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 drop").is_ok());
    assert_eq!(vec![1], f.stack());
}

#[test]
#[ignore]
fn drop_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("drop"));
}

#[test]
#[ignore]
fn swap() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 swap").is_ok());
    assert_eq!(vec![2, 1], f.stack());
}

#[test]
#[ignore]
fn swap_with_three() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 3 swap").is_ok());
    assert_eq!(vec![1, 3, 2], f.stack());
}

#[test]
#[ignore]
fn swap_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("1 swap"));
    assert_eq!(Err(Error::StackUnderflow), f.eval("swap"));
}

#[test]
#[ignore]
fn over() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 over").is_ok());
    assert_eq!(vec![1, 2, 1], f.stack());
}

#[test]
#[ignore]
fn over_with_three() {
    let mut f = Forth::new();
    assert!(f.eval("1 2 3 over").is_ok());
    assert_eq!(vec![1, 2, 3, 2], f.stack());
}

#[test]
#[ignore]
fn over_error() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::StackUnderflow), f.eval("1 over"));
    assert_eq!(Err(Error::StackUnderflow), f.eval("over"));
}

#[test]
#[ignore]
fn defining_a_new_word() {
    let mut f = Forth::new();
    assert!(f.eval(": CoUnT 1 2 3 ;").is_ok());
    assert!(f.eval("count COUNT").is_ok());
    assert_eq!(vec![1, 2, 3, 1, 2, 3], f.stack());
}

#[test]
#[ignore]
fn redefining_an_existing_word() {
    let mut f = Forth::new();
    assert!(f.eval(": foo dup ;").is_ok());
    assert!(f.eval(": foo dup dup ;").is_ok());
    assert!(f.eval("1 foo").is_ok());
    assert_eq!(vec![1, 1, 1], f.stack());
}

#[test]
#[ignore]
fn redefining_an_existing_built_in_word() {
    let mut f = Forth::new();
    assert!(f.eval(": swap dup ;").is_ok());
    assert!(f.eval("1 swap").is_ok());
    assert_eq!(vec![1, 1], f.stack());
}

#[test]
#[ignore]
fn defining_words_with_odd_characters() {
    let mut f = Forth::new();
    assert!(f.eval(": € 220371 ; €").is_ok());
    assert_eq!(vec![220371], f.stack());
}

#[test]
#[ignore]
fn defining_a_number() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::InvalidWord), f.eval(": 1 2 ;"));
}

#[test]
#[ignore]
fn malformed_word_definition() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::InvalidWord), f.eval(":"));
    assert_eq!(Err(Error::InvalidWord), f.eval(": foo"));
    assert_eq!(Err(Error::InvalidWord), f.eval(": foo 1"));
}

#[test]
#[ignore]
fn calling_non_existing_word() {
    let mut f = Forth::new();
    assert_eq!(Err(Error::UnknownWord), f.eval("1 foo"));
}

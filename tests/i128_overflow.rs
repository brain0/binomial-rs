#![cfg(feature = "i128")]
extern crate binomial;
use binomial::Binomial;

#[test]
fn binomial131_no_overflow() {
    assert_eq!(
        130i128.choose(65).unwrap(),
        95067625827960698145584333020095113100i128
    );
}

#[test]
fn binomial132_overflow() {
    assert!(131i128.choose(65).is_none());
}

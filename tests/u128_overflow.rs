#![cfg(feature = "i128")]
extern crate binomial;
use binomial::Binomial;

#[test]
fn binomial131_no_overflow() {
    assert_eq!(
        131u128.choose(65).unwrap(),
        188694833082770476622296176145946360850u128
    );
}

#[test]
fn binomial132_overflow() {
    assert!(132u128.choose(66).is_none());
}

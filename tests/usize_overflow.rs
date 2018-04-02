extern crate binomial;
use binomial::Binomial;

#[test]
fn b67_no_overflow() {
    assert_eq!(67usize.choose(34).unwrap(), 14226520737620288370usize);
}

#[test]
fn b68_overflow() {
    assert!(68usize.choose(34).is_none());
}

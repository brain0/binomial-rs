extern crate binomial;
use binomial::Binomial;

#[test]
fn b66_no_overflow() {
    assert_eq!(66isize.choose(33).unwrap(), 7219428434016265740isize);
}

#[test]
fn b67_overflow() {
    assert!(67isize.choose(34).is_none());
}

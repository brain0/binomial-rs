extern crate binomial;
use binomial::Binomial;

const BINOMIAL_15: [usize; 16] = [
    1, 15, 105, 455, 1365, 3003, 5005, 6435, 6435, 5005, 3003, 1365, 455, 105, 15, 1
];

#[test]
fn b15_usize() {
    for (i, &v) in BINOMIAL_15.iter().enumerate() {
        assert_eq!(15usize.choose(i).unwrap(), v);
    }
}

#[test]
fn b15_u8() {
    for (i, &v) in BINOMIAL_15.iter().enumerate() {
        assert_eq!(15u8.choose(i as u8).unwrap(), v);
    }
}

#[test]
fn b15_u16() {
    for (i, &v) in BINOMIAL_15.iter().enumerate() {
        assert_eq!(15u16.choose(i as u16).unwrap(), v);
    }
}

#[test]
fn b15_u32() {
    for (i, &v) in BINOMIAL_15.iter().enumerate() {
        assert_eq!(15u32.choose(i as u32).unwrap(), v);
    }
}

#[test]
fn b15_u64() {
    for (i, &v) in BINOMIAL_15.iter().enumerate() {
        assert_eq!(15u64.choose(i as u64).unwrap(), v);
    }
}

#[cfg(feature = "i128")]
#[test]
fn b15_u128() {
    for (i, &v) in BINOMIAL_15.iter().enumerate() {
        assert_eq!(15u128.choose(i as u128).unwrap(), v as u128);
    }
}

extern crate binomial;
use binomial::Binomial;

const BINOMIAL_20: [usize; 21] = [
    1, 20, 190, 1140, 4845, 15504, 38760, 77520, 125970, 167960, 184756, 167960, 125970, 77520,
    38760, 15504, 4845, 1140, 190, 20, 1,
];

#[test]
fn b20_usize() {
    for (i, &v) in BINOMIAL_20.iter().enumerate() {
        assert_eq!(20usize.choose(i).unwrap(), v);
    }
}

#[test]
fn b20_u8() {
    for (i, &v) in BINOMIAL_20.iter().enumerate() {
        assert_eq!(20u8.choose(i as u8).unwrap(), v);
    }
}

#[test]
fn b20_u16() {
    for (i, &v) in BINOMIAL_20.iter().enumerate() {
        assert_eq!(20u16.choose(i as u16).unwrap(), v);
    }
}

#[test]
fn b20_u32() {
    for (i, &v) in BINOMIAL_20.iter().enumerate() {
        assert_eq!(20u32.choose(i as u32).unwrap(), v);
    }
}

#[test]
fn b20_u64() {
    for (i, &v) in BINOMIAL_20.iter().enumerate() {
        assert_eq!(20u64.choose(i as u64).unwrap(), v);
    }
}

#[cfg(feature = "i128")]
#[test]
fn b20_u128() {
    for (i, &v) in BINOMIAL_20.iter().enumerate() {
        assert_eq!(20u128.choose(i as u128).unwrap(), v as u128);
    }
}

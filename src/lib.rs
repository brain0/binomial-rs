//! Methods for computing binomial coefficients
//!
//! This crate contains methods for computing generalized binomial coefficients `n choose k`
//! where `k` is an unsigned integer and `n` is either an unsigned integer, a signed integer
//! or a floating point number.
//!
//! For all integer types, the computations is guaranteed not to overflow unless the result
//! would overflow the target type.
//!
//! # Examples
//!
//! For all unsigned integer types, the result is `usize`:
//!
//! ```
//! use binomial::Binomial;
//!
//! let n: u8 = 6;
//!
//! assert_eq!(n.choose(0).unwrap(), 1usize);
//! assert_eq!(n.choose(1).unwrap(), 6usize);
//! assert_eq!(n.choose(2).unwrap(), 15usize);
//! assert_eq!(n.choose(3).unwrap(), 20usize);
//! assert_eq!(n.choose(4).unwrap(), 15usize);
//! assert_eq!(n.choose(5).unwrap(), 6usize);
//! assert_eq!(n.choose(6).unwrap(), 1usize);
//! ```
//!
//! For signed integer types, the result is `isize`:
//!
//! ```
//! # use binomial::Binomial;
//! let n: i8 = -3;
//!
//! assert_eq!(n.choose(0).unwrap(), 1isize);
//! assert_eq!(n.choose(1).unwrap(), -3isize);
//! assert_eq!(n.choose(2).unwrap(), 6isize);
//! assert_eq!(n.choose(3).unwrap(), -10isize);
//! assert_eq!(n.choose(4).unwrap(), 15isize);
//! assert_eq!(n.choose(5).unwrap(), -21isize);
//! ```
//!
//! Computing `67 choose 34` works, but `68 choose 34` overflows `usize`:
//!
//! ```
//! # use binomial::Binomial;
//! assert_eq!(67u8.choose(34).unwrap(), 14226520737620288370);
//! assert!(68u8.choose(34).is_none());
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Trait for types that support the computation of a binomial coefficient.
pub trait Binomial {
    /// The result type.
    type Output;
    /// The type of the second argument
    type SecondArg;
    /// Computes the binomial coefficient.
    ///
    /// This method returns `None` in case of overflow.
    fn choose(self, k: Self::SecondArg) -> Option<Self::Output>;
}

macro_rules! gcd_impl {
    ($name:ident, $type:ty) => {
        fn $name(mut n: $type, mut d: $type) -> $type {
            while d != 0 {
                let r = n % d;
                n = d;
                d = r;
            }
            n
        }
    };
}

gcd_impl!(gcd, usize);
#[cfg(feature = "i128")]
gcd_impl!(gcd128, u128);

macro_rules! mul_divu_impl {
    ($name:ident, $type:ty, $gcd:ident) => {
        fn $name(f1: $type, f2: $type, d: $type) -> Option<$type> {
            let g = $gcd(f1, d);
            (f1 / g).checked_mul(f2 / (d / g))
        }
    };
}

mul_divu_impl!(mul_divu, usize, gcd);
#[cfg(feature = "i128")]
mul_divu_impl!(mul_divu128, u128, gcd128);

macro_rules! unsigned_binomial_impl {
    ($type:ty, $muldiv:ident) => {
        impl Binomial for $type {
            type Output = $type;
            type SecondArg = $type;

            fn choose(self, k: $type) -> Option<$type> {
                if k > self {
                    Some(0)
                } else if self - k < k {
                    self.choose(self - k)
                } else {
                    let mut res: $type = 1;
                    for i in 0..k {
                        match $muldiv(res, self - i, i + 1) {
                            Some(r) => res = r,
                            None => return None,
                        }
                    }
                    Some(res)
                }
            }
        }
    };
}

unsigned_binomial_impl!(usize, mul_divu);
#[cfg(feature = "i128")]
unsigned_binomial_impl!(u128, mul_divu128);

macro_rules! unsigned_binomial_as_usize {
    ( $( $type:ty ),* ) => { $(
        impl Binomial for $type {
            type Output = usize;
            type SecondArg = $type;

            fn choose(self, k: $type) -> Option<usize> {
                (self as usize).choose(k as usize)
            }
        }
    )* }
}
unsigned_binomial_as_usize!(u8, u16, u32, u64);

macro_rules! mul_divi_impl {
    ($name:ident, $type:ty, $unsigned_type:ty, $gcd:ident) => {
        fn $name(f1: $type, f2: $type, d: $type) -> Option<$type> {
            let g = $gcd(f1.abs() as $unsigned_type, d as $unsigned_type) as $type;
            (f1 / g).checked_mul(f2 / (d / g))
        }
    };
}

mul_divi_impl!(mul_divi, isize, usize, gcd);
#[cfg(feature = "i128")]
mul_divi_impl!(mul_divi128, i128, u128, gcd128);

macro_rules! signed_binomial_impl {
    ($type:ty, $muldiv:ident) => {
        impl Binomial for $type {
            type Output = $type;
            type SecondArg = $type;

            fn choose(self, k: $type) -> Option<$type> {
                if k < 0 {
                    Some(0)
                } else {
                    let mut res: $type = 1;
                    for i in 0..k {
                        match $muldiv(res, self - i, i + 1) {
                            Some(r) => res = r,
                            None => return None,
                        }
                    }
                    Some(res)
                }
            }
        }
    };
}
signed_binomial_impl!(isize, mul_divi);
#[cfg(feature = "i128")]
signed_binomial_impl!(i128, mul_divi128);

macro_rules! signed_binomial_as_isize {
    ( $( $type:ty ),* ) => { $(
        impl Binomial for $type {
            type Output = isize;
            type SecondArg = $type;

            fn choose(self, k: $type) -> Option<isize> {
                (self as isize).choose(k as isize)
            }
        }
    )* }
}
signed_binomial_as_isize!(i8, i16, i32, i64);

impl Binomial for f64 {
    type Output = f64;
    type SecondArg = usize;

    fn choose(self, k: usize) -> Option<f64> {
        let mut res = 1f64;
        for i in 0..k {
            res /= (i + 1) as f64;
            res *= self - (i as f64);
        }
        Some(res)
    }
}

impl Binomial for f32 {
    type Output = f64;
    type SecondArg = usize;

    fn choose(self, k: usize) -> Option<f64> {
        (self as f64).choose(k)
    }
}

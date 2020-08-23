use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::Err::Error;
use nom::{
    branch::alt, bytes::complete::take_while, character::complete::char, combinator::map,
    combinator::map_res, sequence::delimited, sequence::pair, IResult,
};

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::gcd;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    Fractional(i64, i64),
    Rounded(f64),
}

impl Number {
    fn reduce(self) -> Self {
        match self {
            Number::Fractional(n, d) => {
                let gcd = gcd(n, d);
                Number::Fractional(n / gcd, d / gcd)
            }
            Number::Rounded(_) => self,
        }
    }
}

impl From<Number> for f64 {
    fn from(item: Number) -> Self {
        match item {
            Number::Fractional(n, d) => n as f64 / d as f64,
            Number::Rounded(v) => v,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = self.reduce();
        match display {
            Number::Fractional(n, 1) => write!(f, "{}", n),
            Number::Fractional(n, d) => write!(f, "{}/{}", n, d),
            Number::Rounded(num) => write!(f, "{}", num),
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        if let Number::Fractional(n, d) = self {
            if let Number::Fractional(rn, rd) = rhs {
                return Number::Fractional(n * rd + rn * d, rd * d);
            }
        }
        Number::Rounded(f64::from(self) + f64::from(rhs))
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        match self {
            Number::Fractional(n, d) => Number::Fractional(-n, d),
            Number::Rounded(n) => Number::Rounded(-n),
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        if let Number::Fractional(a, b) = self {
            if let Number::Fractional(c, d) = rhs {
                return Number::Fractional(a * c, b * d);
            }
        }

        Number::Rounded(f64::from(self) * f64::from(rhs))
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        match rhs {
            Number::Fractional(n, d) => self * Number::Fractional(d, n),
            Number::Rounded(n) => Number::Rounded(f64::from(self) / n),
        }
    }
}

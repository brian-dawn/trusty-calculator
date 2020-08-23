#[macro_use]
extern crate nom;
use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::Err::Error;
use nom::{
    branch::alt, bytes::complete::take_while, character::complete::char, combinator::map,
    combinator::map_res, sequence::delimited, sequence::pair, IResult,
};

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub mod number;
pub mod parser;
pub mod utils;

use number::Number;
use parser::Expr;

pub fn walk(ast: &Expr) -> Number {
    match ast {
        Expr::Add(lhs, rhs) => {
            let lhs = walk(&lhs);
            let rhs = walk(&rhs);
            return lhs + rhs;
        }
        Expr::Sub(lhs, rhs) => {
            let lhs = walk(&lhs);
            let rhs = walk(&rhs);
            return lhs - rhs;
        }
        Expr::Mul(lhs, rhs) => {
            let lhs = walk(&lhs);
            let rhs = walk(&rhs);
            return lhs * rhs;
        }
        Expr::Div(lhs, rhs) => {
            let lhs = walk(&lhs);
            let rhs = walk(&rhs);
            return lhs / rhs;
        }
        Expr::Number(num) => *num,
    }
}

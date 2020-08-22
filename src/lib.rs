#[macro_use]
extern crate nom;
use nom::{
    branch::alt, bytes::complete::take_while, character::complete::char, combinator::map,
    combinator::map_res, sequence::delimited, sequence::pair, IResult,
};

use anyhow::{anyhow, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    Fractional(i64, i64),
    Rounded(f64),
}

impl From<Number> for f64 {
    fn from(item: Number) -> Self {
        match item {
            Number::Fractional(n, d) => n as f64 / d as f64,
            Number::Rounded(v) => v,
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

// 人工の人口です
// the bank on the bank

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

fn is_char_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_float_symbol(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == 'i' || c == 'n' || c == 'f'
}

fn from_float(s: &str) -> std::result::Result<f64, std::num::ParseFloatError> {
    if s.contains(".") || s == "inf" {
        s.parse()
    } else {
        // TODO: figure out how to just return a real parse error here.
        "何".parse()
    }
    // s.contains(".")
    //     .then(s.parse())
    //     .ok_or(std::num::ParseFloatError{

    //     })
}

pub fn parse_integer(input: &str) -> IResult<&str, i64> {
    map_res(take_while(is_char_digit), str::parse)(input)
}

pub fn parse_float(input: &str) -> IResult<&str, f64> {
    map_res(take_while(is_float_symbol), from_float)(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(parse_float, |n| Number::Rounded(n)),
        map(parse_integer, |n| Number::Fractional(n, 1)),
    ))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expr, char(')'))(input)
    // between ( [expr] )
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((map(parse_number, Expr::Number), parse_parens))(input)
    // literal or parens
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    alt((parse_mul, parse_div, parse_factor))(input)
}

pub fn parse_mul(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('*'), parse_term); // * followed by term
    let (input, (lhs, (_, rhs))) = pair(parse_factor, sub)(input)?;
    Ok((input, Expr::Mul(Box::new(lhs), Box::new(rhs))))
}

pub fn parse_div(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('/'), parse_term);
    let (input, (lhs, (_, rhs))) = pair(parse_factor, sub)(input)?;
    Ok((input, Expr::Div(Box::new(lhs), Box::new(rhs))))
}

pub fn parse_sub(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('-'), parse_expr);
    let (input, (lhs, (_, rhs))) = pair(parse_term, sub)(input)?;
    Ok((input, Expr::Sub(Box::new(lhs), Box::new(rhs))))
}

pub fn parse_add(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('+'), parse_expr);
    let (input, (lhs, (_, rhs))) = pair(parse_term, sub)(input)?;
    Ok((input, Expr::Add(Box::new(lhs), Box::new(rhs))))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_add, parse_sub, parse_term))(input)
}

pub fn parse(input: &str) -> Option<Expr> {
    //let (rem, expr) = parse_expr(input).map_err(|_| {
    //    return Err(anyhow!("Nah."));
    //})?;
    let i = input.replace(" ", "");
    let (_, expr) = parse_expr(&i).ok()?;
    Some(expr)
}

// pub fn parse2(input: &str) -> Result<Expr, _> {
//      parse_expr(input).ok_or_else(|| Err("boom"))
// }

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

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Number(Number),
}

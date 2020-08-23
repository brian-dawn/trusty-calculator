use nom::{
    branch::alt, bytes::complete::take_while, character::complete::char, combinator::map,
    combinator::map_res, sequence::delimited, sequence::pair, Err::Failure, IResult,
};

use crate::number::Number;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Number(Number),
}

fn is_char_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_float_symbol(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == 'i' || c == 'n' || c == 'f'
}

fn from_float(s: &str) -> Result<f64, nom::Err<String>> {
    let err = Failure(String::from("Invalid float"));

    if s.contains('.') || s == "inf" {
        s.parse().map_err(|_| err)
    } else {
        Err(err)
    }
}

pub fn parse_integer(input: &str) -> IResult<&str, i64> {
    map_res(take_while(is_char_digit), str::parse)(input)
}

pub fn parse_float(input: &str) -> IResult<&str, f64> {
    map_res(take_while(is_float_symbol), from_float)(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(parse_float, |n| n.into()),
        map(parse_integer, |n| n.into()),
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

pub fn parse(input: &str) -> Result<Expr, String> {
    let i = input.trim().replace(" ", "");
    parse_expr(&i)
        .map_err(|e| format!("{}", e))
        .and_then(|(remaining, parsed)| {
            if !remaining.is_empty() {
                return Err(format!("Parsed: {:?}, Remaining, {}", parsed, remaining));
            }
            Ok(parsed)
        })
}

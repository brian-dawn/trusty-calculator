


use nom::{
    branch::alt, bytes::complete::take_while, character::complete::char, combinator::map,
    combinator::map_res, sequence::delimited, sequence::pair, IResult,
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

pub fn parse(input: &str) -> std::result::Result<Expr, String> {
    let i = input.trim().replace(" ", "");
    parse_expr(&i)
        .map_err(|_| "Oops".to_string())
        .and_then(|(remain, exp)| {
            if !remain.is_empty() {
                Err(String::from("Failed to parse"))
            } else {
                Ok(exp)
            }
        })
}

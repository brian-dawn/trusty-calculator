use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::map,
    combinator::map_res,
    multi::many0,
    sequence::pair,
    sequence::{delimited, preceded},
    Err::Failure,
    IResult,
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

#[derive(Debug)]
pub enum Oper {
    Add,
    Sub,
    Mul,
    Div,
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

fn fold_exprs(initial: Expr, remainder: Vec<(Oper, Expr)>) -> Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        match oper {
            Oper::Add => Expr::Add(Box::new(acc), Box::new(expr)),
            Oper::Sub => Expr::Sub(Box::new(acc), Box::new(expr)),
            Oper::Mul => Expr::Mul(Box::new(acc), Box::new(expr)),
            Oper::Div => Expr::Div(Box::new(acc), Box::new(expr)),
        }
    })
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = parse_factor(input)?;
    let (input, remainder) = many0(alt((
        |i| {
            let (i, mul) = preceded(tag("*"), parse_factor)(i)?;
            Ok((i, (Oper::Mul, mul)))
        },
        |i| {
            let (i, div) = preceded(tag("/"), parse_factor)(i)?;
            Ok((i, (Oper::Div, div)))
        },
    )))(input)?;

    Ok((input, fold_exprs(initial, remainder)))
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, initial) = parse_term(i)?;
    let (i, remainder) = many0(alt((
        |i| {
            let (i, add) = preceded(tag("+"), parse_term)(i)?;
            Ok((i, (Oper::Add, add)))
        },
        |i| {
            let (i, sub) = preceded(tag("-"), parse_term)(i)?;
            Ok((i, (Oper::Sub, sub)))
        },
    )))(i)?;

    Ok((i, fold_exprs(initial, remainder)))
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

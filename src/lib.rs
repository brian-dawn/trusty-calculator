extern crate nom;

pub mod number;
pub mod parser;
pub mod utils;

use number::Number;
use parser::Expr;

pub fn walk(ast: &Expr) -> Number {
    match ast {
        Expr::Add(lhs, rhs) => {
            let lhs = walk(lhs);
            let rhs = walk(rhs);
            lhs + rhs
        }
        Expr::Sub(lhs, rhs) => {
            let lhs = walk(lhs);
            let rhs = walk(rhs);
            lhs - rhs
        }
        Expr::Mul(lhs, rhs) => {
            let lhs = walk(lhs);
            let rhs = walk(rhs);
            lhs * rhs
        }
        Expr::Div(lhs, rhs) => {
            let lhs = walk(lhs);
            let rhs = walk(rhs);
            lhs / rhs
        }
        Expr::Number(num) => *num,
    }
}

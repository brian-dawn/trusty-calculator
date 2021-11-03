#[cfg(test)]
mod tests {
    // use assert_cmd::prelude::*;
    // use std::process::Command;
    use trusty_calculator::number::Number;
    use trusty_calculator::parser::*;

    // TODO
    // fn test_number_cmd() -> Result<()> {
    //     let mut cmd = Command::cargo_bin("trusty-calculator")?;

    //     cmd.arg("3");
    //     cmd.assert().success().stdout("3\n");

    //     Ok(())
    // }

    // TODO
    // fn test_add_cmd() -> Result<()> {
    //     let mut cmd = Command::cargo_bin("trusty-calculator")?;

    //     cmd.arg("1+1");
    //     cmd.assert().success().stdout("2\n");

    //     Ok(())
    // }

    #[test]
    fn test_fraction() {
        let ex = "2/6 * 21";
        let ast = parse(ex).unwrap();
        assert_eq!(
            Expr::Mul(
                Box::new(Expr::Div(
                    Box::new(Expr::Number(Number::Fractional(2, 1))),
                    Box::new(Expr::Number(Number::Fractional(6, 1)),)
                )),
                Box::new(Expr::Number(Number::Fractional(21, 1)))
            ),
            ast
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_integer("1"), Ok(("", 1)));
        assert!(parse_float("1").is_err());
        assert!(parse_integer("-1").is_err());
        assert_eq!(parse_integer("1 + 2"), Ok((" + 2", 1)));
        assert_eq!(parse_integer("12"), Ok(("", 12)));
        assert_eq!(parse_float("1.2"), Ok(("", 1.2)));
        assert!(parse_float("-1.2").is_err());
        assert_eq!(parse_float("inf"), Ok(("", f64::INFINITY)));
        assert!(parse_float("-inf").is_err());

        assert_eq!(parse_number("1"), Ok(("", Number::Fractional(1, 1))));
        assert!(parse_number("-1").is_err());
        assert_eq!(parse_number("1.2"), Ok(("", Number::Fractional(6, 5)))); // <- so fancy

        // TODO
        //assert_eq!(parse_float("∞"), Ok(("", f64::INFINITY)));
    }

    // TODO
    // fn test_negate_parse_expr() {
    //     assert_eq!(parse_expr("-2"), Ok(("", Number::Fractional(-2))));
    //     assert_eq!(parse_expr("-20"), Ok(("", Number::Fractional(-20))));
    //     assert_eq!(parse_expr("-1.2"), Ok(("", Number::Fractional(-1.2))));
    // }
}

use trusty_calculator;

#[cfg(test)]
mod tests {
    use super::trusty_calculator::number::*;
    use super::trusty_calculator::parser::*;
    use super::trusty_calculator::*;

    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_walk() {
        let n = walk(&parse("1+2").unwrap());
        assert_eq!(f64::from(n), 3f64);

        let n = walk(&parse("(1+2)/3").unwrap());
        assert_eq!(f64::from(n), 1f64);

        let n = walk(&parse("(3-0)/3").unwrap());
        assert_eq!(f64::from(n), 1f64);

        let n = walk(&parse("(5-2) / 3").unwrap());
        assert_eq!(f64::from(n), 1f64);

        let n = walk(&parse("inf/inf").unwrap());
        assert!(f64::from(n).is_nan());

        let n = walk(&parse("((1 + 1) * (1 + 1))").unwrap());
        assert_eq!(f64::from(n), 4f64);

        let n = walk(&parse("1 / 2 + 2 / 3").unwrap());
        assert_eq!(n, Number::Fractional(7, 6));
    }
}

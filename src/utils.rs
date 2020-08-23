pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

/// Attempt to convert a decimal to a rational number.
/// https://rosettacode.org/wiki/Convert_decimal_number_to_rational#Rust
#[allow(clippy::many_single_char_names)]
pub fn decimal_to_rational(mut n: f64) -> Option<(i64, i64)> {
    if n.is_nan() || n.is_infinite() {
        return None;
    }

    //Based on Farey sequences
    assert!(n.is_finite());
    let flag_neg = n < 0.0;
    if flag_neg {
        n *= -1.0
    }
    if n < std::f64::MIN_POSITIVE {
        return Some((0, 1));
    }
    if (n - n.round()).abs() < std::f64::EPSILON {
        return Some((n.round() as i64, 1));
    }
    let mut a: i64 = 0;
    let mut b: i64 = 1;
    let mut c: i64 = n.ceil() as i64;
    let mut d: i64 = 1;
    let aux1 = i64::max_value() / 2;
    while c < aux1 && d < aux1 {
        let aux2: f64 = (a as f64 + c as f64) / (b as f64 + d as f64);
        if (n - aux2).abs() < std::f64::EPSILON {
            break;
        }
        if n > aux2 {
            a += c;
            b += d;
        } else {
            c += a;
            d += b;
        }
    }
    // Make sure that the fraction is irreducible
    let gcd = gcd(a + c, b + d);
    if flag_neg {
        Some((-(a + c) / gcd, (b + d) / gcd))
    } else {
        Some(((a + c) / gcd, (b + d) / gcd))
    }
}

#[cfg(test)]
mod tests {

    use super::decimal_to_rational;

    #[test]
    fn test_decimal_to_rational() {
        assert_eq!(decimal_to_rational(0.5), Some((1, 2)));
        assert_eq!(decimal_to_rational(1.0), Some((1, 1)));
        assert_eq!(decimal_to_rational(f64::INFINITY), None);
        assert_eq!(decimal_to_rational(f64::NAN), None);
    }
}

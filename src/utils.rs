pub fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

/// Attempt to convert a decimal to a rational number.
/// https://rosettacode.org/wiki/Convert_decimal_number_to_rational#Rust
pub fn decimal_to_rational(mut n: f64) -> [i64; 2] {
    //Based on Farey sequences
    assert!(n.is_finite());
    let flag_neg = n < 0.0;
    if flag_neg {
        n = n * (-1.0)
    }
    if n < std::f64::MIN_POSITIVE {
        return [0, 1];
    }
    if (n - n.round()).abs() < std::f64::EPSILON {
        return [n.round() as i64, 1];
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
            a = a + c;
            b = b + d;
        } else {
            c = a + c;
            d = b + d;
        }
    }
    // Make sure that the fraction is irreducible
    let gcd = gcd(a + c, b + d);
    if flag_neg {
        [-(a + c) / gcd, (b + d) / gcd]
    } else {
        [(a + c) / gcd, (b + d) / gcd]
    }
}

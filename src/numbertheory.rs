pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = a;
        a = b;
        b = t % b;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(1, gcd(7, 13));
        assert_eq!(101, gcd(9797, 10403));
    }

    #[test]
    fn test_lcm_basic() {
        assert_eq!(91, lcm(7, 13));
        assert_eq!(1009091, lcm(9797, 10403));
    }
}

pub fn is_pythagorean_triple(a: usize, b: usize, c: usize) -> bool {
    if a <= 0 || b <= 0 || c <= 0 {
        return false;
    }
    let mut triples = vec![a, b, c];
    triples.sort();
    triples[2].pow(2) == triples[1].pow(2) + triples[0].pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert!(!is_pythagorean_triple(1, 2, 3));
        assert!(is_pythagorean_triple(3, 4, 5));
    }
}

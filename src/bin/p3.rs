extern crate euler;

fn largest_prime_factor(num: usize) -> usize {
    *euler::prime_factors(num).iter().max().unwrap()
}

fn main() {
    let result = largest_prime_factor(600851475143);
    println!("Largest prime factor of 600851475143: {:?}", result);
}

#[test]
fn test_largest_prime_factor() {
    assert_eq!(29, largest_prime_factor(13195));
}

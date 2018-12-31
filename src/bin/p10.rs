extern crate euler;

fn sum_of_primes(limit: usize) -> usize {
    euler::PrimeSieve::new(limit + 1).iter().sum()
}

fn main() {
    let result = sum_of_primes(2000000);
    println!("Sum of primes below 2M: {:?}", result);
}

#[test]
fn test_sum_of_primes() {
    assert_eq!(17, sum_of_primes(10));
}

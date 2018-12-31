extern crate euler;

use std::collections::HashMap;

// Use the prime tau function to compute the number of factors for given N
fn prime_tau(n: usize) -> usize {
    // List of prime factors
    euler::prime_factors(n)
        .iter()
        // Get the exponent of each prime in a slightly awkward way
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        })
        .iter()
        // https://primes.utm.edu/glossary/page.php?sort=Tau
        .fold(1, |acc, x| (acc * (x.1 + 1 as usize)))
}

fn triangular_number(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn get_first_triangular(limit: usize) -> usize {
    (1..)
        .map(|n| triangular_number(n))
        .map(|n| (n, prime_tau(n)))
        .skip_while(|n| n.1 <= limit)
        .next()
        .unwrap()
        .0
}

fn main() {
    let result = get_first_triangular(500);
    println!("First triangular number with > 500 divisors: {:?}", result)
}

#[test]
fn test_get_first_triangular() {
    assert_eq!(28, get_first_triangular(5));
}

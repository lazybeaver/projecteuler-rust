extern crate euler;

fn prime_upper_bound(index: usize) -> usize {
    // The formula only holds for n > 6
    match index {
        0 => 0,
        1 => 2,
        2 => 3,
        3 => 5,
        4 => 7,
        5 => 11,
        6 => 13,
        // This is a pretty loose but theoretically sound upper bound on the nth prime.
        _ => (index * (((index as f64).ln() + (index as f64).ln().ln()).ceil() as usize)),
    }
}

fn get_prime(index: usize) -> usize {
    euler::PrimeSieve::new(prime_upper_bound(index) + 1)
        .iter()
        .nth(index - 1)  // zero based index
        .unwrap()
}

fn main() {
    let result = get_prime(10001);
    println!("Prime #10001: {:?}", result);
}

#[test]
fn test_get_prime() {
    assert_eq!(2, get_prime(1));
    assert_eq!(3, get_prime(2));
    assert_eq!(5, get_prime(3));
    assert_eq!(7, get_prime(4));
    assert_eq!(11, get_prime(5));
    assert_eq!(13, get_prime(6));
    assert_eq!(97, get_prime(25));
    assert_eq!(541, get_prime(100));
}

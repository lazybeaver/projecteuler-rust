extern crate euler;

fn sum_of_multiples(limit: usize) -> usize {
    (0..limit).filter(|n| (n % 3 == 0 || n % 5 == 0)).sum()
}

fn main() {
    let result = sum_of_multiples(1000);
    println!("Sum of multiples of 3 or 5 less than 1000: {:?}", result);
}

#[test]
fn test_sum_of_multiples() {
    assert_eq!(23, sum_of_multiples(10));
}

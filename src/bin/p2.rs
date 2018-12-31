extern crate euler;

fn sum_of_even_fibonacci(limit: usize) -> usize {
    euler::Fibonacci::new(1, 1)
        .take_while(|n| n < &limit)
        .filter(|n| n % 2 == 0)
        .sum()
}

fn main() {
    let result = sum_of_even_fibonacci(4000000);
    println!("Sum of even fibonacci numbers less than 4M: {:?}", result);
}

#[test]
fn test_sum_of_even_fibonacci() {
    assert_eq!(44, sum_of_even_fibonacci(100));
}

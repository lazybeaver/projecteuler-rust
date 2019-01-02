extern crate num;

use num::bigint::BigInt;
use num::pow;

fn sum_digits(num: BigInt) -> usize {
    num.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .sum()
}

fn main() {
    let result = sum_digits(pow(BigInt::from(2usize), 1000));
    println!("Sum of digits of 2^1000: {:?}", result);
}

#[test]
fn test_sum_of_digits() {
    assert_eq!(26, sum_digits(pow(BigInt::from(2usize), 15)));
}

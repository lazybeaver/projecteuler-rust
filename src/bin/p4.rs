extern crate euler;

fn reverse(original: usize) -> usize {
    let mut n = original;
    let mut reversed = 0;
    while n > 0 {
        let remainder = n % 10;
        reversed = (reversed * 10) + remainder;
        n = n / 10;
    }
    reversed
}

fn is_palindrome(n: usize) -> bool {
    n == reverse(n)
}

fn largest_palindrome() -> usize {
    let mut max = 0;
    for i in 100..999 {
        for j in 100..999 {
            let product = i * j;
            if product > max {
                if is_palindrome(product) {
                    max = product
                }
            }
        }
    }
    max
}

fn main() {
    let result = largest_palindrome();
    println!("Largest palindrome from 3 digit products: {:?}", result);
}

#[test]
fn test_largest_palindrome() {
    assert_eq!(906609, largest_palindrome());
}

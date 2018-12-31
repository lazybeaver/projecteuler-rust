extern crate euler;

fn square_difference(num: usize) -> usize {
    let sum_of_squares: usize = (1..num + 1).map(|n| n * n).sum();
    let square_of_sums: usize = (1..num + 1).sum::<usize>().pow(2);
    square_of_sums - sum_of_squares
}

fn main() {
    let result = square_difference(100);
    println!("Square difference of first 10 integers: {:?}", result);
}

#[test]
fn test_square_difference() {
    assert_eq!(2640, square_difference(10));
}

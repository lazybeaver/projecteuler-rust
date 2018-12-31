extern crate euler;

fn smallest_multiple(nums: Vec<usize>) -> usize {
    nums.iter().fold(1, |acc, n| euler::lcm(acc, *n))
}

fn main() {
    let result = smallest_multiple((1..20).collect());
    println!("Least common multiple of 1..20: {:?}", result);
}

#[test]
fn test_smallest_multiple() {
    assert_eq!(2520, smallest_multiple((1..10).collect()));
}

extern crate euler;

fn collatz_chain_length(num: usize) -> usize {
    let mut cur = num;
    let mut steps = 1;
    while cur != 1 {
        if cur % 2 == 0 {
            cur = cur / 2
        } else {
            cur = (3 * cur) + 1;
        }
        steps += 1;
    }
    steps
}

fn longest_collatz_chain(limit: usize) -> usize {
    (1..(limit + 1))
        .map(|n| (n, collatz_chain_length(n)))
        .max_by(|x, y| (x.1.cmp(&y.1)))
        .unwrap()
        .0
}

fn main() {
    let result = longest_collatz_chain(1000000);
    println!("Number < 1M with longest collatz chain: {:?}", result);
}

#[test]
fn test_collatz_chain_length() {
    assert_eq!(10, collatz_chain_length(13));
}

#[test]
fn test_longest_collatz_chain() {
    // http://oeis.org/A284668
    assert_eq!(9, longest_collatz_chain(10));
    assert_eq!(97, longest_collatz_chain(100));
    assert_eq!(871, longest_collatz_chain(1000));
    assert_eq!(6171, longest_collatz_chain(10000));
    assert_eq!(77031, longest_collatz_chain(100000));
}

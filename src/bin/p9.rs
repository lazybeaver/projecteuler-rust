extern crate euler;

fn special_triple_product() -> usize {
    for a in 1..1000 {
        for b in 1..1000 {
            if a + b >= 1000 {
                continue;
            }
            let c = 1000 - a - b;
            if euler::is_pythagorean_triple(a, b, c) {
                return a * b * c;
            }
        }
    }
    return 0;
}

fn main() {
    let result = special_triple_product();
    println!("Special Pythaogrean Triple Product: {:?}", result);
}

#[test]
fn test_special_triple() {
    assert_eq!(31875000, special_triple_product());
}

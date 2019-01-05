extern crate euler;
extern crate multiarray;

use multiarray::Array2D;

fn parse_input(filename: &str) -> Array2D<usize> {
    let lines = euler::read_data(filename);
    // Only half of the 2D array is used for the triangle.
    // Items in column >= row number will be zero.
    let mut result = Array2D::new([lines.len(), lines.len()], 0);
    for (i, line) in lines.iter().enumerate() {
        for (j, val) in line.split(" ").enumerate() {
            result[[i, j]] = val.parse::<usize>().unwrap();
        }
    }
    result
}

fn maximum_path_sum(filename: &str) -> usize {
    let mut input = parse_input(filename);
    let size = input.extents()[0];
    // Iterate from penultimate row to the top row
    // For each branch a -> (b, c), rewrite a = a + max(b, c)
    for i in (0..size - 1).rev() {
        for j in 0..i + 1 {
            input[[i, j]] += std::cmp::max(input[[i + 1, j]], input[[i + 1, j + 1]]);
        }
    }
    input[[0, 0]]
}

fn main() {
    let result = maximum_path_sum("p18/problem.txt");
    println!("Maximum path sum for triangle is: {}", result);
}

#[test]
fn test_parse_input() {
    let input = parse_input("p18/testcase.txt");
    assert_eq!([4, 4], input.extents());
}

#[test]
fn test_maximum_path_sum() {
    assert_eq!(23, maximum_path_sum("p18/testcase.txt"));
}

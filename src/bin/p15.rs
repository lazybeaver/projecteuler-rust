extern crate euler;
extern crate multiarray;

use multiarray::Array2D;

fn lattice_paths(size: usize) -> usize {
    let mut grid = Array2D::new([size + 1, size + 1], 0);
    // First row and first column have 1 reachable path
    for k in 0..size + 1 {
        grid[[0, k]] = 1;
        grid[[k, 0]] = 1;
    }
    // Every subsequent cell is filled iteratively
    for i in 1..size + 1 {
        for j in 1..size + 1 {
            grid[[i, j]] += grid[[i - 1, j]];
            grid[[i, j]] += grid[[i, j - 1]];
        }
    }
    // The last cell contains the solution
    grid[[size, size]]
}

fn main() {
    let result = lattice_paths(20);
    println!("Number of paths in a 20x20 grid: {:?}", result);
}

#[test]
fn test_lattice_paths() {
    assert_eq!(6, lattice_paths(2));
}

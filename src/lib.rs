// These warnings come from the rust_embed crate.
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// Helper to read embedded input data
#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "data/"]
struct Asset;

pub fn read_data(filename: &str) -> Vec<String> {
    let asset_file = Asset::get(filename).unwrap();
    let contents: &str = std::str::from_utf8(asset_file.as_ref()).unwrap();
    contents
        .to_string()                        // str -> String
        .split("\n")                        // split on newlines
        .map(|x| String::from(x.trim()))    // trim spaces on either ends
        .filter(|x| !x.is_empty())          // filter out empty lines
        .collect()
}

// Import submodules and export public symbols.
mod fibonacci;
pub use fibonacci::Fibonacci;

mod numbertheory;
pub use numbertheory::gcd;
pub use numbertheory::lcm;

mod primes;
pub use primes::prime_factors;
pub use primes::PrimeSieve;

mod trigonometry;
pub use trigonometry::is_pythagorean_triple;

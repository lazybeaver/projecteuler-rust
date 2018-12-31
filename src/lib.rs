// External crates used by modules
extern crate bitvec_rs;

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

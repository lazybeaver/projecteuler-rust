use bitvec_rs::BitVec;

fn isqrt(num: usize) -> usize {
    ((num as f64).sqrt().ceil()) as usize
}

#[derive(Debug)]
pub struct PrimeSieve {
    bits: BitVec,
    limit: usize,
}

impl PrimeSieve {
    pub fn new(limit: usize) -> PrimeSieve {
        // Store only odd numbers in the sieve starting from 3
        let nbits = if limit > 2 { (limit / 2) - 1 } else { 0 };
        let mut sieve = PrimeSieve {
            bits: BitVec::from_elem(nbits, true),
            limit: limit,
        };
        sieve.init(limit);
        sieve
    }

    fn init(&mut self, limit: usize) {
        for num in (3..isqrt(limit)).step_by(2) {
            if self.get(num).unwrap() {
                for multiple in (2..).map(|x| (num * x)).take_while(|x| x < &limit) {
                    self.set(multiple, false)
                }
            }
        }
    }

    fn get(&self, num: usize) -> Option<bool> {
        match num {
            0 | 1 => None,
            2 => Some(true),
            _ => if num % 2 == 0 {
                Some(false)
            } else {
                self.bits.get((num - 3) / 2)
            },
        }
    }

    fn set(&mut self, num: usize, value: bool) {
        if num % 2 != 0 {
            self.bits.set((num - 3) / 2, value);
        }
    }

    fn limit(&self) -> usize {
        self.limit
    }

    pub fn iter<'a>(&'a self) -> PrimeSieveIterator<'a> {
        PrimeSieveIterator {
            sieve: self,
            start: 2,
        }
    }
}

#[derive(Debug)]
pub struct PrimeSieveIterator<'a> {
    sieve: &'a PrimeSieve,
    start: usize,
}

impl<'a> Iterator for PrimeSieveIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        for i in self.start..self.sieve.limit() {
            self.start += 1;
            if self.sieve.get(i).unwrap() {
                return Some(i);
            }
        }
        None
    }
}

pub fn prime_factors(num: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut remainder = num;
    while remainder % 2 == 0 {
        remainder /= 2;
        factors.push(2);
    }
    for i in (3..isqrt(remainder) + 1).step_by(2) {
        while remainder % i == 0 {
            remainder /= i;
            factors.push(i);
        }
    }
    if remainder > 1 {
        factors.push(remainder);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime_sieve_list(limit: usize) -> Vec<usize> {
        PrimeSieve::new(limit + 1).iter().collect()
    }

    fn prime_sieve_count(limit: usize) -> usize {
        PrimeSieve::new(limit + 1).iter().count()
    }

    #[test]
    fn test_prime_sieve_list() {
        assert_eq!(prime_sieve_list(1), []);
        assert_eq!(prime_sieve_list(2), [2]);
        assert_eq!(prime_sieve_list(3), [2, 3]);
        assert_eq!(prime_sieve_list(4), [2, 3]);
        assert_eq!(prime_sieve_list(5), [2, 3, 5]);
        assert_eq!(prime_sieve_list(6), [2, 3, 5]);
        assert_eq!(prime_sieve_list(7), [2, 3, 5, 7]);
        assert_eq!(prime_sieve_list(8), [2, 3, 5, 7]);
        assert_eq!(prime_sieve_list(9), [2, 3, 5, 7]);
        assert_eq!(prime_sieve_list(10), [2, 3, 5, 7]);
        assert_eq!(prime_sieve_list(11), [2, 3, 5, 7, 11]);
        assert_eq!(prime_sieve_list(12), [2, 3, 5, 7, 11]);
        assert_eq!(prime_sieve_list(13), [2, 3, 5, 7, 11, 13]);
        assert_eq!(prime_sieve_list(14), [2, 3, 5, 7, 11, 13]);
        assert_eq!(prime_sieve_list(15), [2, 3, 5, 7, 11, 13]);
        assert_eq!(prime_sieve_list(30), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_prime_sieve_count() {
        assert_eq!(prime_sieve_count(1), 0);
        assert_eq!(prime_sieve_count(10), 4);
        assert_eq!(prime_sieve_count(100), 25);
        assert_eq!(prime_sieve_count(1000), 168);
        assert_eq!(prime_sieve_count(10000), 1229);
        assert_eq!(prime_sieve_count(100000), 9592);
        assert_eq!(prime_sieve_count(1000000), 78498);
        assert_eq!(prime_sieve_count(10000000), 664579);
        assert_eq!(prime_sieve_count(100000000), 5761455);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(1), []);
        assert_eq!(prime_factors(2), [2]);
        assert_eq!(prime_factors(35), [5, 7]);
        assert_eq!(prime_factors(5040), [2, 2, 2, 2, 3, 3, 5, 7]);
        assert_eq!(prime_factors(9797), [97, 101]);
    }
}

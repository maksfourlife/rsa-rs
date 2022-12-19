use num_bigint_dig::{BigUint, RandBigInt};
use num_traits::{FromPrimitive, One, Zero};
use rand::Rng;

fn is_prime(x: &BigUint) -> bool {
    let mut i: BigUint = BigUint::from_u64(2).unwrap();
    let x_sqrt = x.sqrt();
    while i < x_sqrt {
        if x % &i == Zero::zero() {
            return false;
        }
        i = &i + BigUint::one();
    }
    true
}

#[derive(Debug)]
pub struct RandomPrimeIterator {
    m: BigUint,
    x: BigUint,
}

impl RandomPrimeIterator {
    pub fn new(bits: usize) -> Self {
        Self::new_with_rng(bits, &mut rand::thread_rng())
    }

    pub fn new_with_rng<R: Rng>(bits: usize, rng: &mut R) -> Self {
        let x = rng.gen_biguint(bits);
        let m = BigUint::one() << bits;
        Self { m, x }
    }
}

impl Iterator for RandomPrimeIterator {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(&self.x) {
            // self.x is always the same bits
            self.x = (&self.x + BigUint::one()) % &self.m;
        }
        Some(self.x.clone())
    }
}

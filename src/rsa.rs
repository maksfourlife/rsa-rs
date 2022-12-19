use num_bigint_dig::{prime::next_prime, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{one, Signed};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

use crate::math::RandomPrimeIterator;

pub fn gen_keypair(bits: usize) -> (PubKey, PrivKey) {
    gen_keypair_with_rng(bits, &mut rand::thread_rng())
}

pub fn gen_keypair_with_rng<R: Rng>(bits: usize, rng: &mut R) -> (PubKey, PrivKey) {
    let mut rand_primes = RandomPrimeIterator::new_with_rng(bits, rng);
    // let p = rng.gen_prime(bits);
    // let q = rng.gen_prime(bits);
    // nums can be close to each other
    let p = rand_primes.next().unwrap();
    let q = rand_primes.next().unwrap();
    let n = &p * &q;
    let _1: BigUint = one();
    let phi = (&p - &_1) * (&q - &_1);
    let e = next_prime(&rng.gen_biguint_below(&phi));
    let phi = phi.to_bigint().unwrap();
    let mut d = e.to_bigint().unwrap().extended_gcd(&phi).x;
    if d.is_negative() {
        d.add_assign(&phi);
    }
    (
        PubKey { e, n: n.clone() },
        PrivKey {
            d: d.to_biguint().unwrap(),
            n,
        },
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubKey {
    e: BigUint,
    n: BigUint,
}

impl PubKey {
    pub fn encrypt(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        chunked_modpow(data, &self.e, &self.n)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivKey {
    d: BigUint,
    n: BigUint,
}

impl PrivKey {
    pub fn decrypt(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        chunked_modpow(data, &self.d, &self.n)
    }
}

fn chunked_modpow(data: impl AsRef<[u8]>, exponent: &BigUint, modulo: &BigUint) -> Vec<u8> {
    data.as_ref()
        .chunks(modulo.bits() / 8)
        .flat_map(|ch| {
            let c = BigUint::from_bytes_be(ch);
            let m = c.modpow(exponent, modulo);
            m.to_bytes_be()
        })
        .collect()
}

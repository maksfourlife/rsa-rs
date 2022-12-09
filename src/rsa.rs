use num_bigint_dig::{prime::next_prime, BigUint, RandBigInt, RandPrime, ToBigInt};
use num_integer::Integer;
use num_traits::{one, Signed};
use rand::Rng;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct PubKey {
    pub e: BigUint,
    pub n: BigUint,
}

impl PubKey {
    pub fn encrypt(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
        data.as_ref()
            .chunks((self.n.bits() / 8) as usize)
            .flat_map(|ch| {
                let m = BigUint::from_bytes_be(ch);
                let c = m.modpow(&self.e, &self.n);
                c.to_bytes_be()
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Keypair {
    pubkey: PubKey,
    d: BigUint,
}

impl Keypair {
    pub fn new(bits: usize) -> Self {
        Self::new_with_rng(bits, &mut rand::thread_rng())
    }

    pub fn new_with_rng<R: Rng>(bits: usize, rng: &mut R) -> Self {
        let p = rng.gen_prime(bits);
        let q = rng.gen_prime(bits);
        let n = &p * &q;
        let _1: BigUint = one();
        let phi = (&p - &_1) * (&q - &_1);
        let e = next_prime(&rng.gen_biguint_below(&phi));
        let mut d = e
            .to_bigint()
            .unwrap()
            .extended_gcd(&phi.to_bigint().unwrap())
            .x;
        if d.is_negative() {
            d.add_assign(&phi.to_bigint().unwrap());
        }
        Self {
            pubkey: PubKey { e, n },
            d: d.to_biguint().unwrap(),
        }
    }

    pub fn pubkey(&self) -> &PubKey {
        &self.pubkey
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(self.pubkey.n.bits() / 8)
            .flat_map(|ch| {
                let c = BigUint::from_bytes_be(ch);
                let m = c.modpow(&self.d, &self.pubkey.n);
                m.to_bytes_be()
            })
            .collect()
    }
}

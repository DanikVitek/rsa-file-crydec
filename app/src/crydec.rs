use crate::mod_inverse_mod;
use bpn_generator::generator::generate_prime;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;
use std::num::NonZeroU64;

pub struct EncryptionResult {
    pub public_key: BigUint,
    pub private_key: BigUint,
    pub encrypted: BigUint,
}

pub fn encrypt(data: BigUint, key_base_size: NonZeroU64) -> anyhow::Result<EncryptionResult> {
    let p = generate_prime(key_base_size);
    let q = generate_prime(key_base_size);
    let public_key = &p * &q;
    let lambda = (p - BigUint::one()).lcm(&(q - BigUint::one()));

    let mut e: BigUint = 2u8.into();
    while e < lambda && !e.gcd(&lambda).is_one() {
        e += BigUint::one();
    }
    let e = e;

    let private_key = mod_inverse_mod(&e, &lambda)?;
    drop(lambda);

    // encryption
    let encrypted = data.modpow(&e, &public_key);
    drop(e);

    Ok(EncryptionResult {
        public_key,
        private_key,
        encrypted,
    })
}

#[inline]
pub fn decrypt(data: BigUint, public_key: BigUint, private_key: BigUint) -> BigUint {
    data.modpow(&private_key, &public_key)
}

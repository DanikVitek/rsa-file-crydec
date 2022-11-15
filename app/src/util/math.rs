use anyhow::anyhow;
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Zero};

pub fn mod_inverse_mod(number: &BigUint, modulus: &BigUint) -> anyhow::Result<BigUint> {
    let number = BigInt::from_biguint(Sign::Plus, number.clone());
    let modulus = BigInt::from_biguint(Sign::Plus, modulus.clone());
    let mut t = BigInt::zero();
    let mut newt = BigInt::one();
    let mut r = modulus.clone();
    let mut newr = number;

    while !newr.is_zero() {
        let quotient = &r / &newr;
        (t, newt) = (newt.clone(), &t - &(&quotient * &newt));
        (r, newr) = (newr.clone(), &r - &(&quotient * &newr));
    }
    if r > One::one() {
        return Err(anyhow!("a is not invertible"));
    }

    Ok(BigUint::try_from(t.mod_floor(&modulus)).unwrap())
}

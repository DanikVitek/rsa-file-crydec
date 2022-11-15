use num_bigint::BigUint;
use num_traits::{One, Zero};

#[inline]
pub fn log2(num: &BigUint) -> Result<u64, &'static str> {
    let num = num.clone();
    if num.is_zero() {
        return Err("Number must be positive");
    }
    if num.is_one() {
        return Ok(0);
    }
    Ok(num.to_str_radix(2).len() as u64 - 1)
}

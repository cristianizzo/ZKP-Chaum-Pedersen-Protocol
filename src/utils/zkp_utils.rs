use num_bigint::{BigInt, ToBigInt};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use num::Num;
use num_traits::{Zero, One};

pub fn hash_to_bigint<T>(obj: T) -> Result<BigInt, String>
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    let hash = hasher.finish();
    hash.to_bigint()
        .ok_or("Failed to convert hash to BigInt".to_string())
}

pub fn random_big_int(from: BigInt, to: BigInt) -> BigInt {
    rand::thread_rng().gen_bigint_range(&from, &to)
}

pub fn hex_to_bigint(hex: &str) -> BigInt {
    Num::from_str_radix(hex, 16).unwrap()
}

pub fn mod_exp(g: &BigInt, x: &BigInt, q: &BigInt) -> BigInt {

    let one: BigInt = One::one();
    let zero: BigInt = Zero::zero();
    let two: BigInt = &one + &one;

    if q == &one { return zero }
    let mut result = 1.to_bigint().unwrap();

    let mut base = g % q;
    let mut exp = x.clone();
    while &exp > &zero {
        if &exp % &two == one {
            result = result * &base % q;
        }
        exp = exp >> 1;
        base = &base * &base % q
    }

    (result + q) % q
}

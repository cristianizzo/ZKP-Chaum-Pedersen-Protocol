use num::Num;
use num_bigint::{BigInt, RandBigInt, ToBigInt};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use uuid::Uuid;

// Calculates the default hash value of the input object.
// The resulting hash value is converted to a BigInt for compatibility.
pub fn calculate_default_hash<T>(obj: T) -> BigInt
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish().to_bigint().unwrap()
}

// Generates a random BigInt value within the specified range.
pub fn generate_random_bigint(from: BigInt, to: BigInt) -> BigInt {
    rand::thread_rng().gen_bigint_range(&from, &to)
}

// Converts a hexadecimal string representation to a BigInt.
pub fn convert_hex_to_bigint(hex: &str) -> BigInt {
    Num::from_str_radix(hex, 16).unwrap()
}

// Converts a string to a BigInt.
pub fn convert_string_to_bigint(s: &str) -> BigInt {
    BigInt::from_str(s).unwrap()
}

// Generate UUID
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// Performs modular exponentiation using built-in modpow function.
//
// # Arguments
// * `g` - The base as BigInt.
// * `x` - The exponent as BigInt.
// * `q` - The modulus as BigInt.
//
// # Returns
// * Result of (g ^ x) mod q as BigInt.
pub fn perform_modular_exponentiation(g: &BigInt, x: &BigInt, q: &BigInt) -> BigInt {
    g.modpow(x, q)
}

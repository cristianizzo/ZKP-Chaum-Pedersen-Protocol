use num_bigint::{BigInt, ToBigInt, RandBigInt, Sign};

pub fn generate_user_credentials() -> (String, String, BigInt) {

    let mut username = String::new();
    let mut password = String::new();

    println!("Your password equates to the integer x = {}", x);
    println!("Your username = {}", username);

    let x = BigInt::from_bytes_le(Sign::Plus, &mut password.as_bytes());

    (username, password, x)
}

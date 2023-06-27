use std::ops::Rem;

/*
    Functions for modular arithmetic calculations
*/
use num_bigint::BigUint;
use num_traits::{Num};

// prime modulus in Curve25519 is 2^255-19
pub fn get_prime_modulus() -> BigUint {
    BigUint::from_str_radix("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED", 16)
    .expect("Failed to parse Hex")
}

pub fn add(x: BigUint, y: BigUint) -> BigUint {
    (x + y).rem(get_prime_modulus())   
}

pub fn mul(x: BigUint, y: BigUint) -> BigUint {
    (x * y).rem(get_prime_modulus())
}

pub fn test() {
    let x = BigUint::from_str_radix("7A", 16).expect("err");
    let y = BigUint::from_str_radix("7A", 16).expect("err");

    println!("Modular add = {}", add(x, y))

}
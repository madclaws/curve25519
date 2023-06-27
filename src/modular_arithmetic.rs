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

pub fn sub(x: BigUint, y: BigUint) -> BigUint {
    let prime = get_prime_modulus();
    /*
        if y is greater than x, then we get a negative integer, so to alleviate that we use a modular arithmetic property
        
        -y mod p == (p - y) mod p

        substituing above eq in (x - y) mod p, we get (x + (p - y)) mod p
    */
    (x + (&prime - y)).rem(&prime)
}

pub fn test() {
    let x = BigUint::from_str_radix("7B", 16).expect("err");
    let y = BigUint::from_str_radix("7A", 16).expect("err");

    println!("Modular sub = {}", sub(x, y))

}
use std::{ops::{Rem, BitAnd, Shr}, clone};

/*
    Functions for modular arithmetic calculations
*/
use num_bigint::BigUint;
use num_traits::Num;

// prime modulus in Curve25519 is 2^255-19
pub fn get_prime_modulus() -> BigUint {
    BigUint::from_str_radix(
        "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
        16,
    )
    .expect("Failed to parse Hex")
}

pub fn add(x: BigUint, y: BigUint) -> BigUint {
    (x + y).rem(get_prime_modulus())
}

pub fn mul(x: &BigUint, y: &BigUint) -> BigUint {
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

/*
    This is exponentation by squaring

    say we have to calculate x^exp,
    - let result = 1,
    - we go from LSB of the exp to MSB
        - the way we do this is by right shifting exp on every iteration and reassigning new value of exp as it.
    - we do bitwise and, to find out if the current LSB is 1 or not.
    - If 1, then we multiply x with `result`
    - Then we do x = x * x
    - This repeats until the right shitt is over.
 */
pub fn mul_inv(x: BigUint, exp: BigUint) -> BigUint{
    let mut result: BigUint = BigUint::from_str_radix("1", 10).expect("err");
    let mut var_x = x.clone();
    let mut exp_x =  exp.clone();
    let bitandnum: BigUint = BigUint::from(1 as u16);
    loop {
        if exp_x == BigUint::from(0 as u16) {
            return result
        }
        else if exp_x.clone().bitand(&bitandnum) == bitandnum {
            result = result * var_x.clone();
        }
        var_x = mul(&var_x, &var_x);
        exp_x = exp_x.clone().shr(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::modular_arithmetic::*;
    use num_bigint::BigUint;
    use num_traits::{Num, FromPrimitive};

    #[test]
    fn modular_addition() {
        // 122 + 123 = 245, no wrap, ofcourse the prime modulus is sooo big
        let x = BigUint::from_str_radix("7B", 16).expect("err");
        let y = BigUint::from_str_radix("7A", 16).expect("err");
        assert_eq!(add(x, y), BigUint::from_str_radix("F5", 16).expect("err"));

        // 2^255-19 + 2^255-19  = 0, since we wrap the double resulting in rem 0
        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");

        let y = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
        assert_eq!(add(x, y), BigUint::from_str_radix("0", 16).expect("err"));

        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
    
        let y = BigUint::from_str_radix("8000000000000000000000000000000000000000000000000000000000000000", 16).expect("err");
        assert_eq!(
            add(x, y),
            BigUint::from_str_radix("13", 16).expect("err")
        );
    }

    #[test]
    fn modular_multiplication() {
          let x = BigUint::from_str_radix("7B", 16).expect("err");
          let y = BigUint::from_str_radix("7A", 16).expect("err");
          assert_eq!(mul(&x, &y), BigUint::from_str_radix("3A9E", 16).expect("err"));


        // 2^255-19 * 2^255-19  = 0, since we wrap the double resulting in rem 0
        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");

        let y = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
        assert_eq!(mul(&x, &y), BigUint::from_str_radix("0", 16).expect("err"));

        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
    
        let y = BigUint::from_str_radix("8000000000000000000000000000000000000000000000000000000000000000", 16).expect("err");
        assert_eq!(
            mul(&x, &y),
            BigUint::from_str_radix("0", 16).expect("err")
        );
    }

    #[test]
    fn modular_subtraction() {
        let x = BigUint::from_str_radix("7A", 16).expect("err");
        let y = BigUint::from_str_radix("7B", 16).expect("err");
        assert_eq!(sub(x, y), BigUint::from_str_radix("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC", 16).expect("err"));
    
       // 2^255-19 * 2^255-19  = 0, since we wrap the double resulting in rem 0
        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");

        let y = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
        assert_eq!(sub(x, y), BigUint::from_str_radix("0", 16).expect("err"));
        
    }
    
    #[test]
    #[should_panic]
    fn modular_subtraction_panic() {

        // 2 ^ 255-19
        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
    
        // 2 ^ 255
        let y = BigUint::from_str_radix("8000000000000000000000000000000000000000000000000000000000000000", 16).expect("err");
        // this will panic since our prime modulus is smaller than the `b` by 19
        sub(x, y);
    }

    #[test]
    fn multiplication_inverse_exponentiation() {
        let x = BigUint::from_str_radix("2", 10).expect("err");
        let exp = BigUint::from_str_radix("5", 10).expect("err");
        assert_eq!(mul_inv(x, exp), BigUint::from_u32(32).unwrap())
    }
         

}

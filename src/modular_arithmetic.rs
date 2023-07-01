use std::ops::Rem;

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

#[cfg(test)]
mod tests {
    use crate::modular_arithmetic::*;
    use num_bigint::BigUint;
    use num_traits::Num;

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
          assert_eq!(mul(x, y), BigUint::from_str_radix("3A9E", 16).expect("err"));


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
        assert_eq!(mul(x, y), BigUint::from_str_radix("0", 16).expect("err"));

        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");
    
        let y = BigUint::from_str_radix("8000000000000000000000000000000000000000000000000000000000000000", 16).expect("err");
        assert_eq!(
            mul(x, y),
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


         

}

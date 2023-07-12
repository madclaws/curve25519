use std::ops::{BitAnd, BitOr, BitXor, Rem, Shr};

/*
    Functions for modular arithmetic calculations
*/
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num, Zero};

// prime modulus in Curve25519 is 2^255-19
pub fn get_prime_modulus() -> BigUint {
    BigUint::from_str_radix(
        "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
        16,
    )
    .expect("Failed to parse Hex")
}

pub fn add(x: &BigUint, y: &BigUint) -> BigUint {
    (x + y).rem(get_prime_modulus())
}

pub fn mul(x: &BigUint, y: &BigUint) -> BigUint {
    (x * y).rem(get_prime_modulus())
}

pub fn sub(x: &BigUint, y: &BigUint) -> BigUint {
    let prime = get_prime_modulus();
    /*
        if y is greater than x, then we get a negative integer, so to alleviate that we use a modular arithmetic property

        -y mod p == (p - y) mod p

        substituing above eq in (x - y) mod p, we get (x + (p - y)) mod p
    */
    (x + (&prime - y)).rem(&prime)
}

/*
   Modular multiplicative inverse

   x is inverse of num A, if Ax ≅ 1 mod P or Ax mod P = 1

   Since P is prime, we can use fermat's little theorem, where
   a^P ≅ a mod P, multiplying on both sides with a^-2, we get
   a^(P-2) ≅ a^-1 mod P

   So we have to find a^(P-2)
*/
pub fn mul_inv(x: BigUint) -> BigUint {
    let sub = BigUint::from_str_radix("2", 16).expect("Failed to parse Hex");
    calculate_exponent(x, get_prime_modulus() - sub)
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
   - This repeats until exp is 0.
*/
pub fn calculate_exponent(x: BigUint, exp: BigUint) -> BigUint {
    let mut result: BigUint = BigUint::from_u8(1).unwrap();
    let mut var_x = x;
    let mut exp_x = exp;
    let bitandnum: BigUint = BigUint::from(1_u16);
    loop {
        if exp_x == BigUint::from(0_u16) {
            return result;
        } else if exp_x.clone().bitand(&bitandnum) == bitandnum {
            result *= &var_x;
        }
        var_x = mul(&var_x, &var_x);
        exp_x = exp_x.clone().shr(1);
    }
}
/*
   Scalar multiplication of a point's x in a elliptic curve - Montogomery ladder
*/
pub fn mul_k_u(k: BigUint, u: BigUint) -> BigUint {
    let k1 = decode_little_endian(&k);
    // println!("k1: {}", k1);
    let k_and: BigUint = BigUint::from_str_radix(
        "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF8",
        16,
    )
    .expect("Failed to parse");
    let k_or = BigUint::from_str_radix(
        "4000000000000000000000000000000000000000000000000000000000000000",
        16,
    )
    .expect("Failed to parse");
    let k2 = k1.bitand(k_and).bitor(k_or);
    let u1 = decode_little_endian(&u);
    let u_and: BigUint = BigUint::from_str_radix(
        "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        16,
    )
    .expect("Failed to parse");
    let u2 = u1.bitand(u_and);
    mul_k_u_ladder(
        254,
        k2,
        u2.clone(),
        BigUint::from_u8(1).unwrap(),
        Zero::zero(),
        u2,
        BigUint::from_u8(1).unwrap(),
        Zero::zero(),
    )
}

// algorithm for montgomery ladder to calculate scalar multiplication in elliptic curve
fn mul_k_u_ladder(
    mut t: i32,
    k: BigUint,
    x_1: BigUint,
    mut x_2: BigUint,
    mut z_2: BigUint,
    mut x_3: BigUint,
    mut z_3: BigUint,
    mut swap: BigUint,
) -> BigUint {
    loop {
        if t == -1 {
            let (x2a, _x3a) = cswap(&swap, x_2, x_3);
            let (z2a, _z3a) = cswap(&swap, z_2, z_3);
            let inverse = mul_inv(z2a);
            let result = mul(&x2a, &inverse);
            return decode_little_endian(&result);
        } else {
            let kt = k.clone().shr(t).bitand(BigUint::from_u8(1).unwrap());
            let swap_a = swap.bitxor(kt.clone());
            let (x2a, x3a) = cswap(&swap_a, x_2, x_3);
            let (z2a, z3a) = cswap(&swap_a, z_2, z_3);
            let swap_b = kt.clone();
            let a = add(&x2a, &z2a);
            let aa = mul(&a, &a);
            let b = sub(&x2a, &z2a);
            let bb = mul(&b, &b);
            let e = sub(&aa, &bb);
            let c = add(&x3a, &z3a);
            let d = sub(&x3a, &z3a);
            let da = mul(&d, &a);
            let cb = mul(&c, &b);
            let xx1 = add(&da, &cb);
            let x_3b = mul(&xx1, &xx1);
            let xx2 = sub(&da, &cb);
            let xx3 = mul(&xx2, &xx2);
            let z_3b = mul(&x_1, &xx3);
            let x_2b = mul(&aa, &bb);
            let xx4 = mul(&BigUint::from_u32(121665).unwrap(), &e);
            let xx5 = add(&aa, &xx4);
            let z_2b = mul(&e, &xx5);
            t -= 1;
            x_2 = x_2b.clone();
            z_2 = z_2b.clone();
            x_3 = x_3b.clone();
            z_3 = z_3b.clone();
            swap = swap_b.clone();
        }
    }
}

// conditional swap
fn cswap(swap: &BigUint, x2: BigUint, x3: BigUint) -> (BigUint, BigUint) {
    let all_256: BigUint = BigUint::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        16,
    )
    .expect("Failed to parse");
    let dummy = swap * (all_256.bitand(x2.clone().bitxor(x3.clone())));
    let x2a = x2.bitxor(dummy.clone());
    let x3a = x3.bitxor(dummy);
    (x2a, x3a)
}


pub fn decode_little_endian(num: &BigUint) -> BigUint {
    let little_bytes = num.to_bytes_le();
    BigUint::from_bytes_be(&little_bytes)
}

pub fn test_k_u_iter(mut k: BigUint, mut u: BigUint, mut iter: u32) -> String {
    loop {
        if iter == 0 {
            return k.to_str_radix(16);
        } else {
            let u1 = u.clone();
            u = k.clone();
            k = mul_k_u(k, u1);
            iter -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::modular_arithmetic::*;
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num};

    #[test]
    fn modular_addition() {
        // 122 + 123 = 245, no wrap, ofcourse the prime modulus is sooo big
        let x = BigUint::from_str_radix("7B", 16).expect("err");
        let y = BigUint::from_str_radix("7A", 16).expect("err");
        assert_eq!(add(&x, &y), BigUint::from_str_radix("F5", 16).expect("err"));

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
        assert_eq!(add(&x, &y), BigUint::from_str_radix("0", 16).expect("err"));

        let x = BigUint::from_str_radix(
            "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED",
            16,
        )
        .expect("err");

        let y = BigUint::from_str_radix(
            "8000000000000000000000000000000000000000000000000000000000000000",
            16,
        )
        .expect("err");
        assert_eq!(add(&x, &y), BigUint::from_str_radix("13", 16).expect("err"));
    }

    #[test]
    fn modular_multiplication() {
        let x = BigUint::from_str_radix("7B", 16).expect("err");
        let y = BigUint::from_str_radix("7A", 16).expect("err");
        assert_eq!(
            mul(&x, &y),
            BigUint::from_str_radix("3A9E", 16).expect("err")
        );

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

        let y = BigUint::from_str_radix(
            "8000000000000000000000000000000000000000000000000000000000000000",
            16,
        )
        .expect("err");
        assert_eq!(mul(&x, &y), BigUint::from_str_radix("0", 16).expect("err"));
    }

    #[test]
    fn modular_subtraction() {
        let x = BigUint::from_str_radix("7A", 16).expect("err");
        let y = BigUint::from_str_radix("7B", 16).expect("err");
        assert_eq!(
            sub(&x, &y),
            BigUint::from_str_radix(
                "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC",
                16
            )
            .expect("err")
        );

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
        assert_eq!(sub(&x, &y), BigUint::from_str_radix("0", 16).expect("err"));
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
        let y = BigUint::from_str_radix(
            "8000000000000000000000000000000000000000000000000000000000000000",
            16,
        )
        .expect("err");
        // this will panic since our prime modulus is smaller than the `b` by 19
        sub(&x, &y);
    }

    #[test]
    fn exponentiation_by_multiplication() {
        let x = BigUint::from_str_radix("2", 10).expect("err");
        let exp = BigUint::from_str_radix("5", 10).expect("err");
        assert_eq!(calculate_exponent(x, exp), BigUint::from_u32(32).unwrap());
        let xx = BigUint::from_str_radix("1456", 10).expect("err");
        let vec_xx = xx.to_bytes_le();
        assert_eq!(
            BigUint::from_bytes_le(&vec_xx),
            BigUint::from_u32(1456).unwrap()
        )
    }

    #[test]
    fn test_mul_k_u_iter() {
        let k = BigUint::from_str_radix("0900000000000000000000000000000000000000000000000000000000000000", 16).expect("err");
        let u = BigUint::from_str_radix("0900000000000000000000000000000000000000000000000000000000000000", 16).expect("err");
        let res = test_k_u_iter(k, u, 300);
        assert_eq!(
            res,
            String::from("ab01f96be0469f1978174ca1519d0328c40930be793551548917dd2e624ce612")
        )
    }
}

mod modular_arithmetic;
use modular_arithmetic::*;

fn main() {
    println!("Curve25519\n");
    println!("Prime modulus = {}", get_prime_modulus());
    test();
}

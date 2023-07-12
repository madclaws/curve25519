mod modular_arithmetic;
use modular_arithmetic::*;
mod crypto;
use crypto::*;
fn main() {
    println!("\nCurve25519\n");
    println!("Prime modulus used = {:?}\n", get_prime_modulus().to_str_radix(16));

    let (alice_priv_key, alice_pub_key) = create_key_pair();
    let (bob_priv_key, bob_pub_key) = create_key_pair();

    println!("ALICE PRIVATE KEY - {:?}", alice_priv_key);
    println!("ALICE PUBLIC KEY - {:?}\n", alice_pub_key);
    
    println!("BOB PRIVATE KEY - {:?}", bob_priv_key);
    println!("BOB PUBLIC KEY - {:?}\n", bob_pub_key);


    let alice_shared_key = generate_diffie_hellman_key(alice_priv_key, bob_pub_key);
    let bob_shared_key = generate_diffie_hellman_key(bob_priv_key, alice_pub_key);

    println!("ALICE SHARED KEY - {}", alice_shared_key);
    println!("BOB SHARED KEY - {}\n", bob_shared_key);

    let data = "Hello bob";
    let (encrypted_data, tag) = encrypt(data.as_bytes(), alice_shared_key).unwrap();
    println!("ALICE encrypted `{}` into - {:?}\n", data, encrypted_data);
    let decrypted_data = decrypt(&encrypted_data, bob_shared_key, tag);
    println!("BOB decrypted {:?} into {:?}", encrypted_data, String::from_utf8(decrypted_data).unwrap())
}

use num_bigint::BigUint;
use num_traits::{FromPrimitive};
use rand::Rng;
use ring::aead::{UnboundKey, AES_128_GCM, Nonce, SealingKey, BoundKey, NonceSequence, Aad, NONCE_LEN, OpeningKey, Tag};
use crate::modular_arithmetic;

struct IvNonceSequence(u32);

impl NonceSequence for IvNonceSequence {
    
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        let mut nonce_bytes = vec![0; NONCE_LEN];
        let bytes = self.0.to_be_bytes();
        nonce_bytes[8..].copy_from_slice(&bytes);
        self.0 += 1; // advance the counter
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}
pub fn create_key_pair() -> (BigUint, BigUint) {
    // U - A x coordinate in the elliptic curve
    let general_point = BigUint::from_u8(9).unwrap();
    let mut rng = rand::thread_rng();
    // nice trick to create a 256 bit random number, since rust only supports till 128 by default
    let rand_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

    // K 
    let private_key =BigUint::from_bytes_be(&rand_bytes); 

    // K * U
    let public_key = modular_arithmetic::mul_k_u(private_key.clone(), general_point);

    (private_key, public_key)
}

// Ka * public key
pub fn generate_diffie_hellman_key(private_key: BigUint, oth_public_key: BigUint) -> BigUint {
    modular_arithmetic::mul_k_u(private_key, oth_public_key)
}

pub fn encrypt(data: &[u8], shared_key: BigUint) -> Result<(Vec<u8>, Tag), ring::error::Unspecified> {
    let cipher = shared_key.to_bytes_le()[..16].to_vec();
    // let cipher_2 = cipher_1
    let mut key_bytes = [0u8;16];
    key_bytes.copy_from_slice(&cipher);

    let unbound_key = UnboundKey::new(&AES_128_GCM, &key_bytes)?;
    let nonce_sequence = IvNonceSequence(1);

    let mut encrypted_data = Vec::new();
    encrypted_data.extend_from_slice(data);

    let mut sealing_key =  SealingKey::new(unbound_key, nonce_sequence);
    let aad = Aad::empty();
    let tag = sealing_key.seal_in_place_separate_tag(aad, &mut encrypted_data)?;
    
    Ok((encrypted_data, tag))
}

pub fn decrypt(encrypted_data: &[u8], shared_key: BigUint, tag: Tag) -> Vec<u8>{
    let cipher = shared_key.to_bytes_le()[..16].to_vec();
    // let cipher_2 = cipher_1
    let mut key_bytes = [0u8;16];
    key_bytes.copy_from_slice(&cipher);
    
    let dec_unbound_key = UnboundKey::new(&AES_128_GCM, &key_bytes).unwrap();
    let dec_nonce_sequence = IvNonceSequence(1);
    let dec_aad = Aad::empty();

    let mut opening_key = OpeningKey::new(dec_unbound_key, dec_nonce_sequence);
    
    let mut cypher_text_with_tag = [encrypted_data, tag.as_ref()].concat();


    opening_key.open_in_place(dec_aad, &mut cypher_text_with_tag).unwrap().to_vec()
}
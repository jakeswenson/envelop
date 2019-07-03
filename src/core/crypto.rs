use ring::{aead::{AES_256_GCM, NONCE_LEN, MAX_TAG_LEN, self, Aad, Nonce}};
use ring::rand::SecureRandom;
use std::error::Error;

fn create_nonce(rng: &impl SecureRandom) -> Result<ring::aead::Nonce, Box<dyn Error>> {
    let mut nonce = [0u8; NONCE_LEN];
    rng.fill(&mut nonce)?;

    return Ok(ring::aead::Nonce::assume_unique_for_key(nonce));
}

pub fn raw_encrypt(key: &super::Key, data: &[u8], rng: &impl SecureRandom, aad: Option<Vec<u8>>)
                   -> Result<Vec<u8>, Box<dyn Error>> {
    use bytes::{Buf, BufMut};

    let sealing_key = aead::SealingKey::new(&AES_256_GCM, &key.bytes)?;
    let nonce = create_nonce(rng)?;
    let aad = aad.unwrap_or(vec![0u8; 0]);
    let auth_data = Aad::from(&aad);
    let mut output = {
        let mut buff = vec![0u8; NONCE_LEN + data.len() + AES_256_GCM.tag_len()];
        buff.splice(..NONCE_LEN, nonce.as_ref().iter().cloned());
        buff.splice(NONCE_LEN..(NONCE_LEN + data.len()), data.iter().cloned());
        buff
    };
    aead::seal_in_place(&sealing_key, nonce, auth_data, &mut output[NONCE_LEN..], AES_256_GCM.tag_len())?;
    Ok(output)
}

pub fn raw_decrypt(key: &[u8], encrypted_data: &[u8], nonce: Nonce, aad: Option<Vec<u8>>)
                   -> Result<Vec<u8>, Box<dyn Error>> {

    let opening_key = aead::OpeningKey::new(&AES_256_GCM, &key)?;
    let mut cipher_buffer = vec![0u8; encrypted_data.len()];
    cipher_buffer.splice(..encrypted_data.len(), encrypted_data.iter().cloned());

    let aad = aad.unwrap_or(vec![0u8; 0]);
    let auth_data = Aad::from(&aad);
    let result = aead::open_in_place(&opening_key, nonce, Aad::empty(), 0, &mut cipher_buffer)?;

    cipher_buffer.truncate(encrypted_data.len() - AES_256_GCM.tag_len());

    return Ok(cipher_buffer);
}

use std::error::Error;
use std::str::FromStr;
use crate::protos::encryption_result::{EncryptionResult, EncryptedKey, KeyId};
use ring::aead::{Algorithm, Nonce, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};

mod crypto;

#[derive(Clone)]
pub struct Key {
    identifier: KeyId,
    bytes: bytes::Bytes,
}

impl Key {
    pub fn identifier(&self) -> &KeyId {
        return &self.identifier;
    }
}

pub trait Payload {
    fn bytes(&self) -> &[u8];
}

impl Payload for Vec<u8> {
    fn bytes(&self) -> &[u8] {
        return self;
    }
}

impl Payload for &[u8] {
    fn bytes(&self) -> &[u8] {
        return self;
    }
}

fn create_key(rng: &impl SecureRandom, algorithm: &Algorithm, name: String) -> Result<Key, Box<dyn Error>> {
    let mut key_bytes = vec![0u8; algorithm.key_len()];

    rng.fill(&mut key_bytes)?;

    return Ok(Key {
        identifier: KeyId {
            identifier: name,
            ..Default::default()
        },
        bytes: bytes::Bytes::from(key_bytes),
    });
}

pub fn protect_key(key: &Key, encryption_key: &Key, rng: &impl SecureRandom) -> Result<EncryptedKey, Box<dyn Error>> {

    let encrypted_data_key = crypto::raw_encrypt(encryption_key, &key.bytes, rng, None)?;

    let mut encrypted_key_result = EncryptedKey::new();
    encrypted_key_result.set_encryption_key_id(key.identifier().clone());
    encrypted_key_result.set_nonce(Vec::from(&encrypted_data_key[..NONCE_LEN]));
    encrypted_key_result.set_encrypted_key(Vec::from(&encrypted_data_key[NONCE_LEN..]));

    return Ok(encrypted_key_result);
}

pub fn encrypt<Data: Payload>(key: &Key, payload: &Data) -> Result<EncryptionResult, Box<dyn Error>> {
    let mut rng = SystemRandom::new();

    let data_key = create_key(&rng, &AES_256_GCM, "_DK_".to_string())?;
    let encrypted_key_result = protect_key(&data_key, key, &rng)?;

    let encrypted_data = crypto::raw_encrypt(&data_key, payload.bytes(), &rng, None)?;

    let mut result = EncryptionResult::new();

    result.set_key(encrypted_key_result);
    result.set_nonce(Vec::from(&encrypted_data[..NONCE_LEN]));
    result.set_encrypted_data(Vec::from(&encrypted_data[NONCE_LEN..]));

    return Ok(result);
}

pub fn decrypt(mut encrypted_data: EncryptionResult, parent_key: &Key) -> Result<Vec<u8>, Box<dyn Error>> {

    let encrypted_key = encrypted_data.get_key();
    let key_nonce = Nonce::try_assume_unique_for_key(encrypted_key.get_nonce())?;

    let decrypted_data_key = crypto::raw_decrypt(&parent_key.bytes, encrypted_key.get_encrypted_key(), key_nonce, None)?;

    let data_nonce = Nonce::try_assume_unique_for_key(encrypted_data.get_nonce())?;
    let clear_data = crypto::raw_decrypt(&decrypted_data_key, encrypted_data.get_encrypted_data(), data_nonce, None)?;

    return Ok(clear_data);
}

#[cfg(test)]
mod tests {
    use crate::envelop::{encrypt, create_wrapping_key, decrypt, create_key};
    use std::error::Error;
    use std::str::FromStr;
    use protobuf::Message;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let rng = ring::rand::SystemRandom::new();
        let key = create_key(&rng, &ring::aead::AES_256_GCM, "_IK_".to_string())?;
        let clear_bytes = b"hello world";
        let mut encrypted_result = encrypt(&key, &clear_bytes.as_ref())?;
        let serialized = encrypted_result.write_to_bytes()?;
        let deserialized_result: crate::protos::encryption_result::EncryptionResult =
            protobuf::parse_from_bytes(&serialized)?;
        let result = decrypt(deserialized_result, &key)?;
        assert_eq!(result, b"hello world");
        Ok(())
    }
}

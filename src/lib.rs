mod envelop {
    use std::error::Error;
    use std::str::FromStr;

    struct Key {
        identifier: String,
        bytes: Vec<u8>,
    }

    struct ServiceIdentifier(String);
    struct ParentKeyIdentifier(String);
    struct KeyIdentifier(String);

    enum Keys {
        HSMKey(ServiceIdentifier, KeyIdentifier),
        MasterKey(ServiceIdentifier, ParentKeyIdentifier),
        KeyReference(KeyIdentifier, ParentKeyIdentifier),
        DataKey(ParentKeyIdentifier)
    }

    trait Payload {
    }

    trait HSMService {
    }

    enum Services {
        AWS
    }

    impl HSMService for Services {
    }

    trait EnvelopError: Error {
    }

    struct EncryptionResult {
    }

    fn create_hsm_protected_key<Service: HSMService>(hsm_service: Service) -> Result<Key, Box<dyn EnvelopError>>  {
        return Ok(Key { identifier: String::from_str("").unwrap(), bytes: Vec::new() })
    }

    fn create_wrapping_key() -> Key {
        return Key { identifier: String::from_str("").unwrap(),bytes: Vec::new() }
    }

    fn encrypt<Data: Payload>(key: Key, payload: Data) -> Result<EncryptionResult, Box<dyn EnvelopError>> {
        return Ok(EncryptionResult {})
    }

    fn decrypt(encrypted_data: EncryptionResult) -> Result<Vec<u8>, Box<dyn EnvelopError>> {
        return Ok(Vec::new())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

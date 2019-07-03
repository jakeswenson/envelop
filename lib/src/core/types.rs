struct ServiceIdentifier(String);

struct ParentKeyIdentifier(String);

struct KeyIdentifier(String);

enum Keys {
    HSMKey(ServiceIdentifier, KeyIdentifier),
    MasterKey(ServiceIdentifier, ParentKeyIdentifier),
    KeyReference(KeyIdentifier, ParentKeyIdentifier),
    DataKey(ParentKeyIdentifier),
}

trait HSMService {}

enum Services {
    AWS
}

impl HSMService for Services {}

trait EnvelopError: Error {}

fn create_hsm_protected_key<Service: HSMService>(hsm_service: Service) -> Result<Key, Box<dyn EnvelopError>> {
    panic!("TODO");
}

fn create_wrapping_key() -> Key {
    panic!("TODO");
}

use ethereum_private_key_to_address::PrivateKey;
use std::str::FromStr;

fn main() {
    // From str -> Address
    let private_key =
        PrivateKey::from_str("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a")
            .unwrap();
    println!("Address: {}", private_key.address());

    // From slice -> Address
    // Keep in mind that hex::decode doesn't support non-hex characters so you will have to
    // remove the leading 0x
    let private_key =
        hex::decode("92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e").unwrap();
    let private_key = PrivateKey::from_slice(&private_key).unwrap();
    println!("Address: {}", private_key.address());

    // From str -> Public Key
    let private_key =
        PrivateKey::from_str("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a")
            .unwrap();
    println!("Public Key: {}", private_key.public_key());

    // From slice -> Public Key
    let private_key =
        hex::decode("92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e").unwrap();
    let private_key = PrivateKey::from_slice(&private_key).unwrap();
    println!("Public Key: {}", private_key.public_key());
}

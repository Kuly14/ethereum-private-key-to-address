use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKey {
    pub private_key: SecretKey,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrivateKeyParsingError;

impl FromStr for PrivateKey {
    type Err = PrivateKeyParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let priv_key = s.replace("0x", "");

        Ok(Self {
            private_key: SecretKey::from_str(&priv_key).unwrap(),
        })
    }
}

impl From<secp256k1::SecretKey> for PrivateKey {
    fn from(value: secp256k1::SecretKey) -> Self {
        Self {
            private_key: value
        }
    }
}

impl From<&[u8]> for PrivateKey {
    fn from(value: &[u8]) -> Self {
        let private_key = SecretKey::from_slice(value).unwrap();
        Self { private_key }
    }
}

impl From<&[u8; 32]> for PrivateKey {
    fn from(value: &[u8; 32]) -> Self {
        let private_key = SecretKey::from_slice(value).unwrap();
        Self { private_key }
    }
}

impl From<[u8; 32]> for PrivateKey {
    fn from(value: [u8; 32]) -> Self {
        let private_key = SecretKey::from_slice(&value).unwrap();
        Self { private_key }
    }
}

impl From<Vec<u8>> for PrivateKey {
    fn from(value: Vec<u8>) -> Self {
        let private_key = SecretKey::from_slice(&value.to_vec()).unwrap();
        Self { private_key }
    }
}

impl PrivateKey {
    pub fn from_slice(slice: &[u8]) -> Self {
        let private_key = SecretKey::from_slice(slice).unwrap();

        Self { private_key }
    }

    pub fn public_key(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(&public_key.serialize_uncompressed()[1..])
    }

    pub fn public_key_full(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(public_key.serialize_uncompressed())
    }

    pub fn public_key_x(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(&public_key.serialize_uncompressed()[1..33])
    }

    pub fn public_key_y(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(&public_key.serialize_uncompressed()[33..])
    }

    pub fn public_key_slice(&self) -> [u8; 65] {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        public_key.serialize_uncompressed()
    }

    pub fn address(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = self.private_key.public_key(&secp);
        let public_key = public_key.serialize_uncompressed()[1..].to_vec();
        let mut hasher = Keccak256::new();
        hasher.update(public_key);
        let address = hasher.finalize();
        let mut addr = hex::encode(&address[12..32]);
        addr.insert_str(0, "0x");
        addr
    }
}

#[cfg(test)]
pub mod test {
    use std::str::FromStr;
    use hex::FromHex;
    use crate::PrivateKey;

    fn test_account(priv_key: &str, addr: &str) {
        let private_key = PrivateKey::from_str(priv_key).unwrap();
        assert_eq!(
            addr,
            private_key.address()
        );
    }

    #[test]
    fn test_account_one() {
        test_account("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    }

    #[test]
    fn test_account_two() {
        test_account("0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d", "0x70997970c51812dc3a010c7d01b50e0d17dc79c8")
    }
    #[test]
    fn test_account_three() {
        test_account("0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a", "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc")
    }

    #[test]
    fn test_account_four() {
        let private_key =
            Vec::from_hex("8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba")
                .unwrap();
        let private_key = PrivateKey::from(private_key);
        assert_eq!(
            "0x9965507d1a55bcc2695c58ba16fb37d819b0a4dc",
            private_key.address()
        );
    }

    #[test]
    fn test_account_from_slice() {
        let private_key =
            hex::decode("7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6")
                .unwrap();
        let private_key = PrivateKey::from(&private_key[..]);
        assert_eq!(
            "0x90f79bf6eb2c4f870365e785982e1f101e93b906",
            private_key.address()
        );
    }

    #[test]
    fn test_account_from_vec() {
        let private_key = PrivateKey::from(
            hex::decode("47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a")
                .unwrap(),
        );
        assert_eq!(
            "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65",
            private_key.address()
        );
    }

    #[test]
    fn test_account_from_fixed_bytes() {
        let private_key =
            hex::decode("47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a")
                .unwrap();
        let private_key = PrivateKey::from(TryInto::<[u8; 32]>::try_into(private_key).unwrap());
        assert_eq!(
            "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65",
            private_key.address()
        );
    }

    #[test]
    fn test_account_from_slice_custom() {
        let private_key =
            hex::decode("92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e")
                .unwrap();
        let private_key = PrivateKey::from_slice(&private_key);
        assert_eq!(
            "0x976ea74026e726554db657fa54763abd0c3a0aa9",
            private_key.address()
        );
    }
}

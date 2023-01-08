use anyhow::{Context, Result};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

/// PrivateKey struct that contains method that will convert your private key to an ethereum
/// address
///
/// To calculate Ethereum address from your private key follow these steps:
/// ```
/// use std::str::FromStr;
/// use ethereum_private_key_to_address::PrivateKey;
///
/// // 1.) Create PrivateKey struct from str.
/// let private_key = PrivateKey::from_str("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap();
///
/// // 2.) Call the `address()` method on  your private key
/// let address = private_key.address();
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKey {
    /// Private Key
    private_key: SecretKey,
}

impl FromStr for PrivateKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let private_key = s.replace("0x", "");
        let private_key = SecretKey::from_str(&private_key)
            .context("Problem parsing private key, check if your private key is correct")?;

        Ok(Self { private_key })
    }
}

impl From<secp256k1::SecretKey> for PrivateKey {
    fn from(value: secp256k1::SecretKey) -> Self {
        Self { private_key: value }
    }
}

impl From<&[u8]> for PrivateKey {
    fn from(value: &[u8]) -> Self {
        let private_key = SecretKey::from_slice(value).expect("Failed to parse the private key. Check if your encoding to &[u8] is correct and try again. Or you can try the from_str() method");
        Self { private_key }
    }
}

impl From<&[u8; 32]> for PrivateKey {
    fn from(value: &[u8; 32]) -> Self {
        let private_key = SecretKey::from_slice(value).expect("Failed to parse the private key. Check if your encoding to &[u8] is correct and try again. Or you can try the from_str() method");
        Self { private_key }
    }
}

impl From<[u8; 32]> for PrivateKey {
    fn from(value: [u8; 32]) -> Self {
        let private_key = SecretKey::from_slice(&value).expect("Failed to parse the private key. Check if your encoding to &[u8] is correct and try again. Or you can try the from_str() method");
        Self { private_key }
    }
}

impl From<Vec<u8>> for PrivateKey {
    fn from(value: Vec<u8>) -> Self {
        let private_key = SecretKey::from_slice(&value.to_vec()).expect("Failed to parse the private key. Check if your encoding to &[u8] is correct and try again. Or you can try the from_str() method");
        Self { private_key }
    }
}

impl PrivateKey {
    /// Calculates the address from the private key
    /// ```
    /// use ethereum_private_key_to_address::PrivateKey;
    /// use std::str::FromStr;
    ///
    /// let pk = PrivateKey::from_str("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80").unwrap();
    ///
    /// println!("{}", pk.address());
    /// ```
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

    /// Converts your private key in the &[u8] format to PrivateKey struct
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let private_key = SecretKey::from_slice(slice).context("Failed to parse given private key. Make sure your encoding is correct or try the from_str() method")?;

        Ok(Self { private_key })
    }

    /// Returns Full 64 byte Public Key from Private Key without 0x04 in the front as a String. 0x04 is used to
    /// specify the type of the public key. 0x04 in front means the public key is uncompressed
    pub fn public_key(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(&public_key.serialize_uncompressed()[1..])
    }

    /// Returns Full 65 byte Public Key including the prefix as a String. In this case prefix is 0x04. 0x04 is used
    /// to specify the type of the public key. If you want to get public key without the prefix
    /// call the `public_key()` method.
    pub fn public_key_full(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(public_key.serialize_uncompressed())
    }

    /// Returns the x-coordiante of the public key as a string.
    pub fn public_key_x(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(&public_key.serialize_uncompressed()[1..33])
    }

    /// Returns the y-coordinate of the public key
    pub fn public_key_y(&self) -> String {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        hex::encode(&public_key.serialize_uncompressed()[33..])
    }

    /// Returns the entire public key in [u8; 65] format
    pub fn public_key_slice(&self) -> [u8; 65] {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.private_key);
        public_key.serialize_uncompressed()
    }
}

#[cfg(test)]
pub mod test {
    use crate::PrivateKey;
    use hex::FromHex;
    use std::str::FromStr;

    fn test_account(priv_key: &str, addr: &str) {
        let private_key = PrivateKey::from_str(priv_key).unwrap();
        assert_eq!(addr, private_key.address());
    }

    #[test]
    fn test_account_one() {
        test_account(
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
        );
    }

    #[test]
    fn test_account_two() {
        test_account(
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
            "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
        )
    }
    #[test]
    fn test_account_three() {
        test_account(
            "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
            "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc",
        )
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
            private_key.unwrap().address()
        );
    }
}

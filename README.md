# Ethereum Private Key To Address

This library will calculate ethereum address from a private key.

[Documentation](https://docs.rs/ethereum-private-key-to-address/0.1.0/ethereum_private_key_to_address/)

## Calculate Address

Step by step example to generate address:

1. Add `ethereum-private-key-to-address` to your `Cargo.toml`:
```toml
[dependencies]
ethereum-private-key-to-address = "0.1"
```
2. Import `PrivateKey` struct into your project:
```rust
use ethereum_private_key_to_address::PrivateKey;
```
3. Create `PrivateKey` struct with one of these methods:
```rust
use ethereum_private_key_to_address::PrivateKey;

let private_key = PrivateKey::from_str("<your private key as string>").unwrap();

let private_key = PrivateKey::from_slice("<your private key as &[u8]>").unwrap();
```

4. Call the `address()` method on the `PrivateKey` struct:
```rust
use ethereum_private_key_to_address::PrivateKey;

let private_key = PrivateKey::from_str("<your private key as string>").unwrap();

// This will calculate your address from the given private key
let address = private_key.address();

println!("{}", address);
```

## Calculate Public Key

Step by step example to generate public key:

1. Add `ethereum-private-key-to-address` to your `Cargo.toml`:
```toml
[dependencies]
ethereum-private-key-to-address = "0.1"
```
2. Import `PrivateKey` struct into your project:
```rust
use ethereum_private_key_to_address::PrivateKey;
```
3. Create `PrivateKey` struct with one of these methods:
```rust
use ethereum_private_key_to_address::PrivateKey;

let private_key = PrivateKey::from_str("<your private key as string>").unwrap();

let private_key = PrivateKey::from_slice("<your private key as &[u8]>").unwrap();
```

4. There are multiple options when it comes to the public key. Since public key consists of: prefix: 1 byte, x-coordinate: 32 bytes, y-coordinate: 32 bytes. Here are all the methods you can use.
```rust
use ethereum_private_key_to_address::PrivateKey;

let private_key = PrivateKey::from_str("<your private key as string>").unwrap();

// Returns Full 65 byte Public Key including the prefix as a String. In this case prefix is 0x04. 
// 0x04 is used to specify the type of the public key.
let full_public_key_with_prefix = private_key.public_key_full();

// Returns Full 64 byte Public Key from Private Key without 0x04 in the front as a String. 
// 0x04 is used to specify the type of the public key. 0x04 in front means the public key is uncompressed.
let full_public_key = private_key.public_key();

// Returns the x-coordiante of the public key as a string.
let x_coordinate = private_key.public_key_x();

// Returns the y-coordinate of the public key.
let y_coordiante = private_key.public_key_y();

// Returns the entire public key in [u8; 65] format.
let public_key_slice = private_key.public_key_slice();
```

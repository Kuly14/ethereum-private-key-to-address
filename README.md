# Ethereum Private Key To Address

This library will calculate ethereum address from a private key.

## Example

Step by step example:

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

let private_key = PrivateKey::from_str("<your private key as string>");

// This will calculate your address from the given private key
let address = private_key.address();

println!("{}", address);
```

### Todo
- [x] Better error handling
- [x] Documentation

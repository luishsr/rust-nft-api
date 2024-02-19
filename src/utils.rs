use secp256k1::{Message, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::error::Error;

pub fn mock_sign_data(data: &[u8], private_key_hex: &str) -> Result<String, Box<dyn Error>> {
    // Decode the hex private key
    let private_key = SecretKey::from_slice(&hex::decode(private_key_hex)?)?;

    // Create a new Secp256k1 context
    let secp = Secp256k1::new();

    // Hash the data using Keccak256
    let data_hash = Keccak256::digest(data);

    // Sign the hash
    let message = Message::from_digest_slice(&data_hash)?;
    let signature = secp.sign_ecdsa(&message, &private_key);

    // Encode the signature as hex
    Ok(hex::encode(signature.serialize_compact()))
}

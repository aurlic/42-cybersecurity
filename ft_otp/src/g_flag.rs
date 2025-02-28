use crate::error::OTPError;
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes256;
// use hex::{decode, encode};
use rand::RngCore;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn generate_random_key() -> Result<Vec<u8>, OTPError> {
    let mut rng = rand::rng();
    let mut key = vec![0u8; 32];
    rng.fill_bytes(&mut key);

    let mut file = File::create("aes.key").map_err(|_| OTPError::EncryptionError)?;
    file.write_all(&key)
        .map_err(|_| OTPError::EncryptionError)?;

    Ok(key)
}

pub fn handle_g(key: String) -> Result<(), OTPError> {
    let path = Path::new(&key);
    if !path.exists() {
        return Err(OTPError::FileNotFound(key));
    }

    let content = fs::read_to_string(&key).map_err(|_| OTPError::FileNotFound(key.clone()))?;
    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        return Err(OTPError::FileEmpty(key));
    }
    if trimmed_content.len() < 64 || !is_hex(trimmed_content) {
        return Err(OTPError::InvalidKeyFormat);
    }
    let trimmed_content = &trimmed_content[0..64];

    let encryption_key = generate_random_key()?;

    let key_bytes = hex::decode(&trimmed_content).map_err(|_| OTPError::InvalidKeyFormat)?;

    let cipher = Aes256::new(GenericArray::from_slice(&encryption_key));

    let mut encrypted_data = Vec::new();
    for chunk in key_bytes.chunks(16) {
        let mut block = GenericArray::from([0u8; 16]);
        for (i, &byte) in chunk.iter().enumerate() {
            block[i] = byte;
        }

        cipher.encrypt_block(&mut block);
        encrypted_data.extend_from_slice(&block);
    }

    let mut file = File::create("ft_otp.key").map_err(|_| OTPError::EncryptionError)?;
    file.write_all(&encrypted_data)
        .map_err(|_| OTPError::EncryptionError)?;

    println!("✅ Key file '{}' is valid and saved securely!", key);
    Ok(())
}

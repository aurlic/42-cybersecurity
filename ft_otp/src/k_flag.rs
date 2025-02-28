use crate::error::OTPError;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes256;
use hmac::Hmac;
use sha1::Sha1;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

pub fn handle_k(_key_file: String) -> Result<(), OTPError> {
    let encrypted_key =
        fs::read("ft_otp.key").map_err(|_| OTPError::FileNotFound("ft_otp.key".to_string()))?;

    let encryption_key =
        fs::read("aes.key").map_err(|_| OTPError::FileNotFound("aes.key".to_string()))?;

    let cipher = Aes256::new(GenericArray::from_slice(&encryption_key));

    let mut decrypted_key = Vec::new();
    for chunk in encrypted_key.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted_key.extend_from_slice(&block);
    }

    let counter = get_counter()?;

    let otp = generate_hotp(&decrypted_key, counter)?;

    println!("{:06}", otp);

    Ok(())
}

fn get_counter() -> Result<u64, OTPError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| OTPError::EncryptionError)?
        .as_secs();

    Ok(now / 30)
}

fn generate_hotp(key: &[u8], counter: u64) -> Result<u32, OTPError> {
    let counter_bytes = counter.to_be_bytes();

    let hmac = compute_hmac_sha1(key, &counter_bytes)?;

    let offset = (hmac[19] & 0xf) as usize;
    let binary = ((hmac[offset] & 0x7f) as u32) << 24
        | ((hmac[offset + 1] & 0xff) as u32) << 16
        | ((hmac[offset + 2] & 0xff) as u32) << 8
        | ((hmac[offset + 3] & 0xff) as u32);

    let otp = binary % 1_000_000;

    Ok(otp)
}

fn compute_hmac_sha1(key: &[u8], message: &[u8]) -> Result<[u8; 20], OTPError> {
    use hmac::Mac;

    let mut mac =
        <HmacSha1 as hmac::Mac>::new_from_slice(key).map_err(|_| OTPError::EncryptionError)?;

    mac.update(message);

    let result = mac.finalize().into_bytes();

    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&result[..20]);

    Ok(bytes)
}

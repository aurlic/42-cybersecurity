use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OTPError {
    FileNotFound(String),
    FileEmpty(String),
    InvalidKeyFormat,
    EncryptionError,
}

impl fmt::Display for OTPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OTPError::FileNotFound(path) => write!(f, "File '{}' not found.", path),
            OTPError::FileEmpty(path) => write!(f, "File '{}' is empty.", path),
            OTPError::InvalidKeyFormat => write!(f, "Key must be 64 hexadecimal characters."),
            OTPError::EncryptionError => write!(f, "Encryption failed"),
        }
    }
}

impl Error for OTPError {}

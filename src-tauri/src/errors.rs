use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptVaultError {
    #[error("Key derivation failed: {0}")]
    Kdf(String),

    #[error("Encryption failed")]
    Cipher(String),

    /// Deliberately generic — never distinguish decoy/hidden/nonexistent
    #[error("Invalid password or corrupted container")]
    DecryptionFailed,

    #[error("Invalid container format or corrupted data")]
    InvalidContainer,

    #[error("Container is too small to hold the requested content")]
    InsufficientSpace,

    #[error("Hidden volume would overlap with decoy volume")]
    VolumeOverlap,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("KDF parameters below minimum safe threshold")]
    WeakKdfParams,
}

impl serde::Serialize for CryptVaultError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

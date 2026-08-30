//! AES-256-GCM authenticated encryption/decryption.
//!
//! All encryption in CryptVault uses AES-256-GCM (authenticated):
//! - Provides confidentiality AND integrity/authenticity
//! - A tampered ciphertext will fail decryption with an error
//! - NEVER use AES-CBC (unauthenticated) for any purpose in this codebase
//!
//! SECURITY: Nonces must NEVER be reused with the same key.
//! Each encryption generates a fresh random nonce. Store the nonce
//! alongside the ciphertext (it is not secret, only unique).

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use rand::rngs::OsRng as RandOsRng;
use zeroize::Zeroizing;

use crate::errors::CryptVaultError;

/// Generates a fresh random 12-byte nonce (96 bits) suitable for AES-256-GCM.
/// Each encryption operation MUST use a unique nonce.
pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    RandOsRng.fill_bytes(&mut nonce);
    nonce
}

/// Encrypts `plaintext` with AES-256-GCM using the given 32-byte key and nonce.
///
/// Returns the ciphertext + 16-byte GCM authentication tag (appended by the crate).
///
/// # Arguments
/// * `key`       - 32-byte AES-256 key (derived from Argon2id)
/// * `nonce`     - 12-byte nonce (MUST be unique per key; store alongside ciphertext)
/// * `plaintext` - Data to encrypt
///
/// # Errors
/// Returns `CryptVaultError::Cipher` on failure (key or nonce size mismatch).
pub fn encrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptVaultError> {
    let cipher_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(cipher_key);
    let gcm_nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(gcm_nonce, plaintext)
        .map_err(|_| CryptVaultError::Cipher("Encryption failed".into()))
}

/// Decrypts `ciphertext` with AES-256-GCM. The GCM tag is verified before
/// returning any plaintext — a tampered byte causes an error.
///
/// # Errors
/// Returns `CryptVaultError::Cipher` if the tag verification fails (wrong key,
/// wrong nonce, or tampered data). The error message deliberately does NOT reveal
/// which of these caused the failure.
pub fn decrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Zeroizing<Vec<u8>>, CryptVaultError> {
    let cipher_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(cipher_key);
    let gcm_nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(gcm_nonce, ciphertext)
        .map_err(|_| CryptVaultError::DecryptionFailed)?;

    Ok(Zeroizing::new(plaintext))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] { [0xABu8; 32] }
    fn test_nonce() -> [u8; 12] { [0x01u8; 12] }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"Hello, CryptVault! This is a test message.";

        let ciphertext = encrypt(&key, &nonce, plaintext).unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"secret data";

        let ciphertext = encrypt(&key, &nonce, plaintext).unwrap();

        let wrong_key = [0xFFu8; 32];
        let result = decrypt(&wrong_key, &nonce, &ciphertext);
        assert!(result.is_err(), "Wrong key must not decrypt successfully");
    }

    #[test]
    fn test_tampered_ciphertext_fails() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"integrity test";

        let mut ciphertext = encrypt(&key, &nonce, plaintext).unwrap();
        // Flip a bit in the middle of the ciphertext
        let mid = ciphertext.len() / 2;
        ciphertext[mid] ^= 0xFF;

        let result = decrypt(&key, &nonce, &ciphertext);
        assert!(result.is_err(), "Tampered ciphertext must fail authentication");
    }

    #[test]
    fn test_wrong_nonce_fails() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"nonce test";

        let ciphertext = encrypt(&key, &nonce, plaintext).unwrap();

        let wrong_nonce = [0xFFu8; 12];
        let result = decrypt(&key, &wrong_nonce, &ciphertext);
        assert!(result.is_err(), "Wrong nonce must fail authentication");
    }

    #[test]
    fn test_generate_nonce_unique() {
        let n1 = generate_nonce();
        let n2 = generate_nonce();
        // Statistically virtually impossible to collide; if this fails, OsRng is broken
        assert_ne!(n1, n2, "Nonces must be unique");
    }

    #[test]
    fn test_ciphertext_length() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"length check";
        let ct = encrypt(&key, &nonce, plaintext).unwrap();
        // AES-GCM appends a 16-byte tag
        assert_eq!(ct.len(), plaintext.len() + 16);
    }
}

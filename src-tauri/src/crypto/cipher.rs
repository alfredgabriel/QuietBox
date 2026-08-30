use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Key, Nonce};
use rand::RngCore;
use rand::rngs::OsRng;
use zeroize::Zeroizing;
use crate::errors::CryptVaultError;

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, CryptVaultError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher
        .encrypt(Nonce::from_slice(nonce), plaintext)
        .map_err(|_| CryptVaultError::Cipher("Encryption failed".into()))
}

pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Zeroizing<Vec<u8>>, CryptVaultError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let pt = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| CryptVaultError::DecryptionFailed)?;
    Ok(Zeroizing::new(pt))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn k() -> [u8; 32] { [0xABu8; 32] }
    fn n() -> [u8; 12] { [0x01u8; 12] }

    #[test]
    fn test_roundtrip() {
        let ct = encrypt(&k(), &n(), b"hello cryptvault").unwrap();
        let pt = decrypt(&k(), &n(), &ct).unwrap();
        assert_eq!(pt.as_slice(), b"hello cryptvault");
    }

    #[test]
    fn test_wrong_key_fails() {
        let ct = encrypt(&k(), &n(), b"secret").unwrap();
        assert!(decrypt(&[0xFFu8; 32], &n(), &ct).is_err());
    }

    #[test]
    fn test_tamper_fails() {
        let mut ct = encrypt(&k(), &n(), b"integrity").unwrap();
        ct[4] ^= 0xFF;
        assert!(decrypt(&k(), &n(), &ct).is_err());
    }

    #[test]
    fn test_wrong_nonce_fails() {
        let ct = encrypt(&k(), &n(), b"nonce").unwrap();
        assert!(decrypt(&k(), &[0xFFu8; 12], &ct).is_err());
    }

    #[test]
    fn test_ciphertext_length() {
        let pt = b"length";
        let ct = encrypt(&k(), &n(), pt).unwrap();
        assert_eq!(ct.len(), pt.len() + 16); // 16-byte GCM tag
    }
}

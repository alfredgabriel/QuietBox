//! Key Derivation Functions for CryptVault.
//!
//! Uses Argon2id (memory-hard KDF) for password-to-key derivation.
//! Uses BLAKE3 for deterministic offset derivation from a derived key.
//!
//! SECURITY: Never log or expose derived keys. All key material uses zeroize.

use argon2::{
    password_hash::SaltString,
    Argon2, ParamsBuilder, Version, Algorithm,
};
use blake3::Hasher;
use rand::RngCore;
use rand::rngs::OsRng;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::errors::CryptVaultError;

/// Argon2id parameters. Stored (encrypted) in the volume header so future
/// versions can migrate parameters without breaking existing containers.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KdfParams {
    /// Memory cost in kibibytes (default: 262144 = 256 MiB)
    pub m_cost: u32,
    /// Time cost / iterations (default: 3)
    pub t_cost: u32,
    /// Parallelism (default: 4)
    pub p_cost: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            m_cost: 262_144, // 256 MiB
            t_cost: 3,
            p_cost: 4,
        }
    }
}

impl KdfParams {
    /// Minimum safe parameters. If the user tries to go below this,
    /// the UI must show a strong warning and require explicit confirmation.
    pub fn minimum_safe() -> Self {
        Self {
            m_cost: 65_536, // 64 MiB
            t_cost: 2,
            p_cost: 2,
        }
    }

    pub fn is_below_minimum(&self) -> bool {
        self.m_cost < Self::minimum_safe().m_cost
            || self.t_cost < Self::minimum_safe().t_cost
            || self.p_cost < Self::minimum_safe().p_cost
    }
}

/// A 32-byte derived key. Zeroized on drop to prevent key material leaking in RAM.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey(pub [u8; 32]);

/// Generates a cryptographically secure random 32-byte salt using OsRng.
/// Each volume must have its own unique salt — never reuse between volumes or files.
pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Derives a 256-bit key from a password and salt using Argon2id.
///
/// # Arguments
/// * `password` - The user-supplied password (UTF-8 bytes)
/// * `salt` - A unique 32-byte salt for this volume
/// * `params` - Argon2id parameters
///
/// # Security
/// The derived key is wrapped in `DerivedKey` which zeroizes on drop.
pub fn derive_key(
    password: &[u8],
    salt: &[u8; 32],
    params: &KdfParams,
) -> Result<DerivedKey, CryptVaultError> {
    let argon2_params = ParamsBuilder::new()
        .m_cost(params.m_cost)
        .t_cost(params.t_cost)
        .p_cost(params.p_cost)
        .output_len(32)
        .build()
        .map_err(|e| CryptVaultError::Kdf(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);

    let mut output = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut output)
        .map_err(|e| CryptVaultError::Kdf(e.to_string()))?;

    Ok(DerivedKey(output))
}

/// Derives the hidden volume byte offset deterministically from a derived key.
///
/// Uses BLAKE3(key || "QUIETBOX_HIDDEN_OFFSET_V1") → truncated to u64.
/// The result is bounded to a safe region within the container by the caller.
///
/// This means: without the hidden password, the offset is mathematically
/// unknowable. It is never stored anywhere in plaintext.
pub fn derive_hidden_offset(key: &DerivedKey) -> u64 {
    let mut hasher = Hasher::new();
    hasher.update(&key.0);
    hasher.update(b"QUIETBOX_HIDDEN_OFFSET_V1");
    let hash = hasher.finalize();
    let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
    u64::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_deterministic() {
        let password = b"test-password-123";
        let salt = [0x42u8; 32];
        // Use fast params for tests
        let params = KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 };

        let key1 = derive_key(password, &salt, &params).unwrap();
        let key2 = derive_key(password, &salt, &params).unwrap();
        assert_eq!(key1.0, key2.0, "Same inputs must yield same key");
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let salt = [0x42u8; 32];
        let params = KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 };

        let key1 = derive_key(b"password_a", &salt, &params).unwrap();
        let key2 = derive_key(b"password_b", &salt, &params).unwrap();
        assert_ne!(key1.0, key2.0, "Different passwords must yield different keys");
    }

    #[test]
    fn test_different_salts_different_keys() {
        let password = b"same-password";
        let params = KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 };
        let salt1 = [0x01u8; 32];
        let salt2 = [0x02u8; 32];

        let key1 = derive_key(password, &salt1, &params).unwrap();
        let key2 = derive_key(password, &salt2, &params).unwrap();
        assert_ne!(key1.0, key2.0, "Different salts must yield different keys");
    }

    #[test]
    fn test_hidden_offset_deterministic() {
        let params = KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 };
        let salt = [0x55u8; 32];
        let key = derive_key(b"hidden-pw", &salt, &params).unwrap();
        let off1 = derive_hidden_offset(&key);
        let off2 = derive_hidden_offset(&key);
        assert_eq!(off1, off2, "Offset must be deterministic from key");
    }

    #[test]
    fn test_hidden_offset_differs_per_key() {
        let params = KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 };
        let salt = [0x55u8; 32];
        let key_a = derive_key(b"pw_a", &salt, &params).unwrap();
        let key_b = derive_key(b"pw_b", &salt, &params).unwrap();
        assert_ne!(derive_hidden_offset(&key_a), derive_hidden_offset(&key_b));
    }

    #[test]
    fn test_generate_salt_unique() {
        let s1 = generate_salt();
        let s2 = generate_salt();
        assert_ne!(s1, s2, "Salts must be unique per call");
    }

    #[test]
    fn test_params_minimum_check() {
        let safe = KdfParams::default();
        assert!(!safe.is_below_minimum());
        let unsafe_params = KdfParams { m_cost: 4096, t_cost: 1, p_cost: 1 };
        assert!(unsafe_params.is_below_minimum());
    }
}

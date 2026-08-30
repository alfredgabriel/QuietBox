use argon2::{ParamsBuilder, Argon2, Version, Algorithm};
use blake3::Hasher;
use rand::RngCore;
use rand::rngs::OsRng;
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::errors::CryptVaultError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Zeroize)]
pub struct KdfParams {
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self { m_cost: 262_144, t_cost: 3, p_cost: 4 }
    }
}

impl KdfParams {
    pub fn minimum_safe() -> Self {
        Self { m_cost: 65_536, t_cost: 2, p_cost: 2 }
    }

    pub fn is_below_minimum(&self) -> bool {
        let min = Self::minimum_safe();
        self.m_cost < min.m_cost || self.t_cost < min.t_cost || self.p_cost < min.p_cost
    }
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey(pub [u8; 32]);

pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

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

    fn fast_params() -> KdfParams { KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 } }

    #[test]
    fn test_derive_key_deterministic() {
        let salt = [0x42u8; 32];
        let k1 = derive_key(b"pw", &salt, &fast_params()).unwrap();
        let k2 = derive_key(b"pw", &salt, &fast_params()).unwrap();
        assert_eq!(k1.0, k2.0);
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let salt = [0x42u8; 32];
        let k1 = derive_key(b"pw_a", &salt, &fast_params()).unwrap();
        let k2 = derive_key(b"pw_b", &salt, &fast_params()).unwrap();
        assert_ne!(k1.0, k2.0);
    }

    #[test]
    fn test_different_salts_different_keys() {
        let k1 = derive_key(b"pw", &[0x01u8; 32], &fast_params()).unwrap();
        let k2 = derive_key(b"pw", &[0x02u8; 32], &fast_params()).unwrap();
        assert_ne!(k1.0, k2.0);
    }

    #[test]
    fn test_hidden_offset_deterministic() {
        let k = derive_key(b"hidden", &[0x55u8; 32], &fast_params()).unwrap();
        assert_eq!(derive_hidden_offset(&k), derive_hidden_offset(&k));
    }

    #[test]
    fn test_hidden_offset_differs_per_key() {
        let k1 = derive_key(b"pw_a", &[0x55u8; 32], &fast_params()).unwrap();
        let k2 = derive_key(b"pw_b", &[0x55u8; 32], &fast_params()).unwrap();
        assert_ne!(derive_hidden_offset(&k1), derive_hidden_offset(&k2));
    }

    #[test]
    fn test_generate_salt_unique() {
        assert_ne!(generate_salt(), generate_salt());
    }

    #[test]
    fn test_params_minimum_check() {
        assert!(!KdfParams::default().is_below_minimum());
        assert!(KdfParams { m_cost: 4096, t_cost: 1, p_cost: 1 }.is_below_minimum());
    }
}

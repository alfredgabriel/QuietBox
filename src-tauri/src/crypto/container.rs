use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::crypto::kdf::KdfParams;

pub const HEADER_BLOCK_SIZE: usize = 512;
pub const DECOY_HEADER_OFFSET: u64 = 0;
pub const SAFETY_GAP: u64 = 4096;
pub const FORMAT_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct VolumeHeader {
    pub version: u8,
    pub salt: [u8; 32],
    pub data_nonce: [u8; 12],
    pub data_size: u64,
    pub data_checksum: [u8; 32],
    pub hidden_max_size: u64,
    pub kdf_params: KdfParams,
}

impl VolumeHeader {
    pub fn to_bytes(&self) -> Result<Vec<u8>, crate::errors::CryptVaultError> {
        bincode::serialize(self)
            .map_err(|e| crate::errors::CryptVaultError::Serialization(e.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::errors::CryptVaultError> {
        bincode::deserialize(bytes)
            .map_err(|_| crate::errors::CryptVaultError::InvalidContainer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> VolumeHeader {
        VolumeHeader {
            version: FORMAT_VERSION,
            salt: [0xAAu8; 32],
            data_nonce: [0xBBu8; 12],
            data_size: 1024 * 1024,
            data_checksum: [0xCCu8; 32],
            hidden_max_size: 0,
            kdf_params: KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 },
        }
    }

    #[test]
    fn test_header_roundtrip() {
        let h = sample();
        let bytes = h.to_bytes().unwrap();
        let h2 = VolumeHeader::from_bytes(&bytes).unwrap();
        assert_eq!(h2.version, FORMAT_VERSION);
        assert_eq!(h2.salt, [0xAAu8; 32]);
        assert_eq!(h2.data_nonce, [0xBBu8; 12]);
        assert_eq!(h2.data_size, 1024 * 1024);
    }

    #[test]
    fn test_corrupted_bytes_fail() {
        // A truncated buffer (1 byte) can never deserialize a full VolumeHeader.
        // This is more reliable than XOR-flipping all bytes because bincode
        // does not have internal checksums — flipped bytes can still decode.
        let truncated = &[0xFFu8; 1];
        assert!(VolumeHeader::from_bytes(truncated).is_err(),
            "Truncated buffer must not deserialize into a valid VolumeHeader");

        // Also verify a completely empty input fails
        assert!(VolumeHeader::from_bytes(&[]).is_err(),
            "Empty buffer must not deserialize");
    }
}

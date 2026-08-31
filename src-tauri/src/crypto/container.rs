use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};
use std::io::{Read, Seek, SeekFrom, Write};
use rand::RngCore;
use rand::rngs::OsRng;
use subtle::ConstantTimeEq;

use crate::crypto::kdf::{KdfParams, DerivedKey, derive_key, generate_salt};
use crate::crypto::cipher::{encrypt, decrypt, generate_nonce};
use crate::errors::CryptVaultError;

/// Fixed size of an encrypted header block on disk (bytes).
pub const HEADER_BLOCK_SIZE: usize = 512;

/// Byte offset where the decoy volume header is always written.
pub const DECOY_HEADER_OFFSET: u64 = 0;

/// Minimum safety gap (bytes).
pub const SAFETY_GAP: u64 = 65_536; // 64 KiB

/// Format version stored (encrypted) in headers. Never visible externally.
pub const FORMAT_VERSION: u8 = 1;

/// Chunk size for streaming I/O (1 MiB).
pub const CHUNK_SIZE: usize = 1024 * 1024;

/// The decrypted volume header. Zeroized on drop.
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
    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptVaultError> {
        bincode::serialize(self)
            .map_err(|e| CryptVaultError::Serialization(e.to_string()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptVaultError> {
        bincode::deserialize(bytes).map_err(|_| CryptVaultError::InvalidContainer)
    }
}

pub fn blake3_hash(data: &[u8]) -> [u8; 32] {
    let hash = blake3::hash(data);
    let mut out = [0u8; 32];
    out.copy_from_slice(hash.as_bytes());
    out
}

pub fn seal_header(header: &VolumeHeader, key: &DerivedKey) -> Result<Vec<u8>, CryptVaultError> {
    let header_bytes = header.to_bytes()?;
    let nonce = generate_nonce();
    let ciphertext = encrypt(&key.0, &nonce, &header_bytes)?;

    if 32 + 12 + 2 + ciphertext.len() > HEADER_BLOCK_SIZE {
        return Err(CryptVaultError::Serialization(
            "Serialized header exceeds HEADER_BLOCK_SIZE".into(),
        ));
    }

    let mut block = vec![0u8; HEADER_BLOCK_SIZE];
    block[..32].copy_from_slice(&header.salt);
    block[32..44].copy_from_slice(&nonce);
    
    let ct_len = ciphertext.len() as u16;
    block[44..46].copy_from_slice(&ct_len.to_le_bytes());
    block[46..46 + ciphertext.len()].copy_from_slice(&ciphertext);
    Ok(block)
}

pub fn unseal_header(block: &[u8], key: &DerivedKey) -> Result<VolumeHeader, CryptVaultError> {
    if block.len() < HEADER_BLOCK_SIZE {
        return Err(CryptVaultError::InvalidContainer);
    }
    let nonce: [u8; 12] = block[32..44]
        .try_into()
        .map_err(|_| CryptVaultError::InvalidContainer)?;
    
    let ct_len = u16::from_le_bytes(block[44..46].try_into().map_err(|_| CryptVaultError::InvalidContainer)?) as usize;
    if 46 + ct_len > HEADER_BLOCK_SIZE {
        return Err(CryptVaultError::InvalidContainer);
    }

    let ciphertext = &block[46..46 + ct_len];
    let plaintext = decrypt(&key.0, &nonce, ciphertext)?;
    VolumeHeader::from_bytes(&plaintext)
}

/// Derives the hidden volume offset deterministically from the password and total container size.
/// The hidden volume is positioned in the upper half of the container file.
pub fn derive_hidden_header_offset(
    password: &[u8],
    total_size: u64,
    hidden_max_size: u64,
) -> Result<u64, CryptVaultError> {
    let half_size = total_size / 2;
    let header_sz = HEADER_BLOCK_SIZE as u64;
    
    let min_offset = half_size + SAFETY_GAP;
    let max_offset = total_size
        .checked_sub(hidden_max_size + header_sz)
        .ok_or(CryptVaultError::InsufficientSpace)?;

    if min_offset >= max_offset {
        return Err(CryptVaultError::VolumeOverlap);
    }

    let mut hasher = blake3::Hasher::new_keyed(&[0x4A; 32]);
    hasher.update(b"QUIETBOX_HIDDEN_OFFSET_DOMAIN_V1");
    hasher.update(password);
    let hash = hasher.finalize();
    let raw_bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
    let raw = u64::from_le_bytes(raw_bytes);

    let span = max_offset - min_offset;
    Ok(min_offset + (raw % span))
}

pub fn create_decoy_volume<W: Write + Seek, F: FnMut(f64, &str)>(
    writer: &mut W,
    total_size: u64,
    decoy_data: &[u8],
    decoy_password: &[u8],
    hidden_max_size: u64,
    kdf_params: &KdfParams,
    mut on_progress: F,
) -> Result<u64, CryptVaultError> {
    let max_decoy_allowed = (total_size / 2).saturating_sub(HEADER_BLOCK_SIZE as u64 + 16);
    if decoy_data.len() as u64 > max_decoy_allowed {
        return Err(CryptVaultError::InsufficientSpace);
    }

    on_progress(0.0, "Preparing random fill...");
    writer.seek(SeekFrom::Start(0))?;
    let mut chunk = vec![0u8; CHUNK_SIZE];
    let mut written = 0u64;
    while written < total_size {
        let to_write = CHUNK_SIZE.min((total_size - written) as usize);
        OsRng.fill_bytes(&mut chunk[..to_write]);
        writer.write_all(&chunk[..to_write])?;
        written += to_write as u64;
        
        let p = (written as f64 / total_size as f64) * 0.7;
        on_progress(p, "Filling container with high-entropy noise...");
    }

    on_progress(0.7, "Deriving encryption keys...");
    let salt = generate_salt();
    let key = derive_key(decoy_password, &salt, kdf_params)?;
    let data_nonce = generate_nonce();
    
    on_progress(0.8, "Encrypting decoy volume...");
    let encrypted_data = encrypt(&key.0, &data_nonce, decoy_data)?;
    let checksum = blake3_hash(decoy_data);

    let header = VolumeHeader {
        version: FORMAT_VERSION,
        salt,
        data_nonce,
        data_size: decoy_data.len() as u64,
        data_checksum: checksum,
        hidden_max_size,
        kdf_params: kdf_params.clone(),
    };

    on_progress(0.9, "Writing decoy headers...");
    let header_block = seal_header(&header, &key)?;
    writer.seek(SeekFrom::Start(DECOY_HEADER_OFFSET))?;
    writer.write_all(&header_block)?;
    writer.write_all(&encrypted_data)?;

    let decoy_end = DECOY_HEADER_OFFSET
        + HEADER_BLOCK_SIZE as u64
        + encrypted_data.len() as u64;
    writer.flush()?;
    
    on_progress(1.0, "Decoy volume created successfully");
    Ok(decoy_end)
}

pub fn add_hidden_volume<RW: Read + Write + Seek, F: FnMut(f64, &str)>(
    rw: &mut RW,
    total_size: u64,
    _decoy_end: u64,
    hidden_data: &[u8],
    hidden_password: &[u8],
    hidden_max_size: u64,
    kdf_params: &KdfParams,
    mut on_progress: F,
) -> Result<(), CryptVaultError> {
    if hidden_data.len() as u64 > hidden_max_size {
        return Err(CryptVaultError::InsufficientSpace);
    }

    on_progress(0.05, "Deriving hidden keys...");
    let salt = generate_salt();
    let key = derive_key(hidden_password, &salt, kdf_params)?;
    
    on_progress(0.2, "Locating hidden volume offset...");
    let offset = derive_hidden_header_offset(hidden_password, total_size, hidden_max_size)?;

    on_progress(0.4, "Encrypting hidden volume...");
    let data_nonce = generate_nonce();
    let encrypted_data = encrypt(&key.0, &data_nonce, hidden_data)?;
    let checksum = blake3_hash(hidden_data);

    let header = VolumeHeader {
        version: FORMAT_VERSION,
        salt,
        data_nonce,
        data_size: hidden_data.len() as u64,
        data_checksum: checksum,
        hidden_max_size,
        kdf_params: kdf_params.clone(),
    };

    on_progress(0.7, "Writing hidden headers...");
    let header_block = seal_header(&header, &key)?;
    rw.seek(SeekFrom::Start(offset))?;
    rw.write_all(&header_block)?;
    rw.write_all(&encrypted_data)?;
    rw.flush()?;
    
    on_progress(1.0, "Hidden volume created successfully");
    Ok(())
}

pub struct OpenResult {
    pub plaintext: Vec<u8>,
    pub is_hidden: bool,
}

pub fn open_container<R: Read + Seek>(
    reader: &mut R,
    total_size: u64,
    password: &[u8],
    kdf_params: &KdfParams,
) -> Result<OpenResult, CryptVaultError> {
    // --- Decoy Volume Check ---
    let mut decoy_block = vec![0u8; HEADER_BLOCK_SIZE];
    reader.seek(SeekFrom::Start(DECOY_HEADER_OFFSET))?;
    reader.read_exact(&mut decoy_block)?;

    let decoy_salt: [u8; 32] = decoy_block[..32]
        .try_into()
        .map_err(|_| CryptVaultError::InvalidContainer)?;

    let decoy_key = derive_key(password, &decoy_salt, kdf_params)?;

    let decoy_result: Result<OpenResult, CryptVaultError> = (|| {
        let header = unseal_header(&decoy_block, &decoy_key)?;
        let data_offset = DECOY_HEADER_OFFSET + HEADER_BLOCK_SIZE as u64;
        reader.seek(SeekFrom::Start(data_offset))?;
        let enc_size = header.data_size as usize + 16;
        let mut enc = vec![0u8; enc_size];
        reader.read_exact(&mut enc)?;
        let pt = decrypt(&decoy_key.0, &header.data_nonce, &enc)?;
        let computed = blake3_hash(&pt);
        if !bool::from(computed.ct_eq(&header.data_checksum)) {
            return Err(CryptVaultError::DecryptionFailed);
        }
        Ok(OpenResult { plaintext: pt.to_vec(), is_hidden: false })
    })();

    // --- Hidden Volume Check (Always executed for constant-time behavior) ---
    let hidden_max_estimate = total_size / 4;
    let candidate_offset = derive_hidden_header_offset(password, total_size, hidden_max_estimate).ok();

    let hidden_result: Result<OpenResult, CryptVaultError> = candidate_offset
        .ok_or(CryptVaultError::DecryptionFailed)
        .and_then(|offset| {
            let mut hblock = vec![0u8; HEADER_BLOCK_SIZE];
            reader.seek(SeekFrom::Start(offset))?;
            reader.read_exact(&mut hblock)?;
            let hsalt: [u8; 32] = hblock[..32]
                .try_into()
                .map_err(|_| CryptVaultError::InvalidContainer)?;
            let hkey = derive_key(password, &hsalt, kdf_params)?;
            let hheader = unseal_header(&hblock, &hkey)?;
            let data_offset = offset + HEADER_BLOCK_SIZE as u64;
            reader.seek(SeekFrom::Start(data_offset))?;
            let enc_size = hheader.data_size as usize + 16;
            let mut enc = vec![0u8; enc_size];
            reader.read_exact(&mut enc)?;
            let pt = decrypt(&hkey.0, &hheader.data_nonce, &enc)?;
            let computed = blake3_hash(&pt);
            if !bool::from(computed.ct_eq(&hheader.data_checksum)) {
                return Err(CryptVaultError::DecryptionFailed);
            }
            Ok(OpenResult { plaintext: pt.to_vec(), is_hidden: true })
        });

    if decoy_result.is_ok() {
        decoy_result
    } else if hidden_result.is_ok() {
        hidden_result
    } else {
        Err(CryptVaultError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn fast_kdf() -> KdfParams { KdfParams { m_cost: 8192, t_cost: 1, p_cost: 1 } }

    fn sample_header(salt: [u8; 32]) -> VolumeHeader {
        VolumeHeader {
            version: FORMAT_VERSION,
            salt,
            data_nonce: [0xBBu8; 12],
            data_size: 16,
            data_checksum: [0xCCu8; 32],
            hidden_max_size: 0,
            kdf_params: fast_kdf(),
        }
    }

    #[test]
    fn test_header_roundtrip() {
        let salt = [0xAAu8; 32];
        let h = sample_header(salt);
        let bytes = h.to_bytes().unwrap();
        let h2 = VolumeHeader::from_bytes(&bytes).unwrap();
        assert_eq!(h2.version, FORMAT_VERSION);
        assert_eq!(h2.salt, [0xAAu8; 32]);
        assert_eq!(h2.data_size, 16);
    }

    #[test]
    fn test_corrupted_bytes_fail() {
        assert!(VolumeHeader::from_bytes(&[0xFFu8; 1]).is_err());
        assert!(VolumeHeader::from_bytes(&[]).is_err());
    }

    #[test]
    fn test_seal_unseal_roundtrip() {
        let salt = generate_salt();
        let key = derive_key(b"test-pw", &salt, &fast_kdf()).unwrap();
        let h = sample_header(salt);
        let block = seal_header(&h, &key).unwrap();
        assert_eq!(block.len(), HEADER_BLOCK_SIZE);
        let h2 = unseal_header(&block, &key).unwrap();
        assert_eq!(h2.version, FORMAT_VERSION);
        assert_eq!(h2.data_size, 16);
    }

    #[test]
    fn test_seal_wrong_key_fails() {
        let salt = generate_salt();
        let key = derive_key(b"correct-pw", &salt, &fast_kdf()).unwrap();
        let wrong_key = derive_key(b"wrong-pw", &salt, &fast_kdf()).unwrap();
        let h = sample_header(salt);
        let block = seal_header(&h, &key).unwrap();
        assert!(unseal_header(&block, &wrong_key).is_err());
    }

    #[test]
    fn test_hidden_offset_bounded() {
        let total = 10 * 1024 * 1024u64;
        let hidden_max = 2 * 1024 * 1024u64;
        let offset = derive_hidden_header_offset(b"hidden-pw", total, hidden_max).unwrap();
        let safe_start = total / 2 + SAFETY_GAP;
        let safe_end = total - hidden_max - HEADER_BLOCK_SIZE as u64;
        assert!(offset >= safe_start);
        assert!(offset <= safe_end);
    }

    #[test]
    fn test_create_decoy_volume() {
        let total = 5 * 1024 * 1024u64;
        let mut buf = vec![0u8; total as usize];
        let mut cur = Cursor::new(&mut buf);
        let decoy_end = create_decoy_volume(
            &mut cur, total, b"decoy content", b"decoy-pw", 1024 * 1024, &fast_kdf(), |_, _| {}
        ).unwrap();
        assert_eq!(buf.len(), total as usize);
        assert!(decoy_end > HEADER_BLOCK_SIZE as u64);
    }

    #[test]
    fn test_container_random_fill_entropy() {
        let total = 1024 * 1024u64;
        let mut buf = vec![0u8; total as usize];
        let mut cur = Cursor::new(&mut buf);
        create_decoy_volume(&mut cur, total, b"data", b"pw", 0, &fast_kdf(), |_, _| {}).unwrap();
        let zero_count = buf.iter().filter(|&&b| b == 0).count();
        let zero_ratio = zero_count as f64 / total as f64;
        assert!(zero_ratio < 0.01,
            "Random fill should have <1% zero bytes, got {:.2}%", zero_ratio * 100.0);
    }

    #[test]
    fn test_integration_decoy_and_hidden() {
        let total = 8 * 1024 * 1024u64; // 8 MiB
        let mut buf = vec![0u8; total as usize];
        let mut cur = Cursor::new(&mut buf);

        let decoy_pw = b"decoy_pass_123";
        let hidden_pw = b"hidden_pass_456";
        let decoy_data = b"This is the DECOY data volume!";
        let hidden_data = b"This is the HIGHLY SENSITIVE HIDDEN data volume!";

        // 1. Create decoy volume
        let _ = create_decoy_volume(
            &mut cur,
            total,
            decoy_data,
            decoy_pw,
            2 * 1024 * 1024,
            &fast_kdf(),
            |_, _| {}
        ).unwrap();

        // 2. Add hidden volume
        add_hidden_volume(
            &mut cur,
            total,
            0,
            hidden_data,
            hidden_pw,
            2 * 1024 * 1024,
            &fast_kdf(),
            |_, _| {}
        ).unwrap();

        // 3. Open with decoy password
        let res_decoy = open_container(&mut cur, total, decoy_pw, &fast_kdf()).unwrap();
        assert!(!res_decoy.is_hidden);
        assert_eq!(res_decoy.plaintext, decoy_data);

        // 4. Open with hidden password
        let res_hidden = open_container(&mut cur, total, hidden_pw, &fast_kdf()).unwrap();
        assert!(res_hidden.is_hidden);
        assert_eq!(res_hidden.plaintext, hidden_data);

        // 5. Open with incorrect password
        let res_wrong = open_container(&mut cur, total, b"wrongpassword", &fast_kdf());
        assert!(res_wrong.is_err());
    }
}

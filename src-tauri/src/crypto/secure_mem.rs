//! Wrappers for handling cryptographic material safely in memory.
//!
//! All structs in this module implement `Zeroize` and `ZeroizeOnDrop`
//! to ensure key material is wiped from RAM as soon as it is no longer needed.
//! This is critical: a key that lingers in memory is a security leak.

use zeroize::{Zeroize, ZeroizeOnDrop};

/// A fixed-size secret buffer that is zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretBytes<const N: usize>(pub [u8; N]);

impl<const N: usize> SecretBytes<N> {
    pub fn new(data: [u8; N]) -> Self {
        Self(data)
    }
}

/// A variable-length secret buffer that is zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretVec(pub Vec<u8>);

impl SecretVec {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

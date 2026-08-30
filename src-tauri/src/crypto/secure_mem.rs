use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretBytes<const N: usize>(pub [u8; N]);

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretVec(pub Vec<u8>);

impl SecretVec {
    pub fn new(data: Vec<u8>) -> Self { Self(data) }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
}

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "ink"))]
mod hash;
#[cfg(not(feature = "ink"))]
pub use hash::*;

#[cfg(feature = "ink")]
pub use ink_hash::*;

#[cfg(feature = "ink")]
mod ink_hash {
    use ink_env::hash::CryptoHash;
    use light_bitcoin_primitives::{H160, H256, H32};

    /// RIPEMD160
    #[inline]
    pub fn ripemd160(_input: &[u8]) -> H160 {
        unimplemented!()
    }

    /// SHA-1
    #[inline]
    pub fn sha1(_input: &[u8]) -> H160 {
        unimplemented!()
    }

    /// SHA-256
    #[inline]
    pub fn sha256(input: &[u8]) -> H256 {
        let mut output: [u8; 32] = Default::default();
        ink_env::hash::Sha2x256::hash(input, &mut output);
        output.into()
    }

    /// SHA-256 and RIPEMD160
    #[inline]
    pub fn dhash160(_input: &[u8]) -> H160 {
        unimplemented!()
    }

    /// Double SHA-256
    #[inline]
    pub fn dhash256(input: &[u8]) -> H256 {
        let mut output: [u8; 32] = Default::default();
        ink_env::hash::Sha2x256::hash(input, &mut output);
        let mut output2: [u8; 32] = Default::default();
        ink_env::hash::Sha2x256::hash(&output, &mut output2);
        output2.into()
    }

    /// SipHash-2-4
    #[inline]
    pub fn siphash24(_key0: u64, _key1: u64, _input: &[u8]) -> u64 {
        unimplemented!()
    }

    /// Data checksum
    #[inline]
    pub fn checksum(data: &[u8]) -> H32 {
        H32::from_slice(&dhash256(data)[0..4])
    }
}

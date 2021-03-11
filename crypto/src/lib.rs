#![cfg_attr(not(feature = "std"), no_std)]

use light_bitcoin_primitives::H256;

#[cfg(not(feature = "ink"))]
mod hash;
#[cfg(not(feature = "ink"))]
pub use hash::*;

#[cfg(feature = "ink")]
pub fn dhash256(input: &[u8]) -> H256 {
    use ink_env::hash::CryptoHash;
    let mut output: [u8; 32] = Default::default();
    ink_env::hash::Sha2x256::hash(input, &mut output);
    let mut output2: [u8; 32] = Default::default();
    ink_env::hash::Sha2x256::hash(&output, &mut output2);
    output2.into()
}

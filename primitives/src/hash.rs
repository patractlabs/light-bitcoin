pub use primitive_types::{H160, H512};

use fixed_hash::construct_fixed_hash;
use impl_codec::impl_fixed_hash_codec;
#[cfg(all(feature = "std", feature = "scale-info"))]
use impl_serde::impl_fixed_hash_serde;

#[cfg(feature = "ink")]
use array_init::array_init;
#[cfg(feature = "ink")]
use ink_primitives::Key;
#[cfg(feature = "ink")]
use ink_storage::traits::{KeyPtr, PackedLayout, SpreadLayout};

#[cfg(all(feature = "std", feature = "ink"))]
use ink_metadata::layout::{ArrayLayout, Layout, LayoutKey};
#[cfg(all(feature = "std", feature = "ink"))]
use ink_storage::traits::{ExtKeyPtr, StorageLayout};

#[cfg(feature = "scale-info")]
use scale_info::{MetaType, Type, TypeDefArray, TypeInfo};

construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 4 bytes (32 bits) size.
    #[cfg_attr(feature = "ink", derive(PackedLayout, SpreadLayout))]
    #[cfg_attr(all(feature = "std", feature = "scale-info"), derive(TypeInfo))]
    #[cfg_attr(all(feature = "std", feature = "ink"), derive(StorageLayout))]
    pub struct H32(4);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    #[cfg_attr(feature = "ink", derive(PackedLayout, SpreadLayout))]
    #[cfg_attr(all(feature = "std", feature = "scale-info"), derive(TypeInfo))]
    #[cfg_attr(all(feature = "std", feature = "ink"), derive(StorageLayout))]
    pub struct H256(32);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 33 bytes (264 bits) size.
    pub struct H264(33);
}

#[cfg(feature = "ink")]
impl SpreadLayout for H264 {
    const FOOTPRINT: u64 = 33 * <u8 as SpreadLayout>::FOOTPRINT;
    const REQUIRES_DEEP_CLEAN_UP: bool = <u8 as SpreadLayout>::REQUIRES_DEEP_CLEAN_UP;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        let a = array_init::<[u8; 33], _>(|_| <u8 as SpreadLayout>::pull_spread(ptr));
        a.into()
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        for elem in &self.0 {
            <u8 as SpreadLayout>::push_spread(elem, ptr)
        }
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        for elem in &self.0 {
            <u8 as SpreadLayout>::clear_spread(elem, ptr)
        }
    }
}

#[cfg(feature = "ink")]
impl PackedLayout for H264 {
    #[inline]
    fn pull_packed(&mut self, at: &Key) {
        for elem in &mut self.0 {
            <u8 as PackedLayout>::pull_packed(elem, at)
        }
    }

    #[inline]
    fn push_packed(&self, at: &Key) {
        for elem in &self.0 {
            <u8 as PackedLayout>::push_packed(elem, at)
        }
    }

    #[inline]
    fn clear_packed(&self, at: &Key) {
        for elem in &self.0 {
            <u8 as PackedLayout>::clear_packed(elem, at)
        }
    }
}

#[cfg(all(feature = "std", feature = "ink"))]
impl StorageLayout for H264 {
    fn layout(key_ptr: &mut KeyPtr) -> Layout {
        let len: u32 = 33;
        let elem_footprint = <u8 as SpreadLayout>::FOOTPRINT;
        Layout::Array(ArrayLayout::new(
            LayoutKey::from(key_ptr.next_for::<H264>()),
            len,
            elem_footprint,
            <u8 as StorageLayout>::layout(&mut key_ptr.clone()),
        ))
    }
}

#[cfg(all(feature = "std", feature = "scale-info"))]
impl TypeInfo for H264 {
    type Identity = Self;

    fn type_info() -> Type {
        TypeDefArray::new(33, MetaType::new::<u8>()).into()
    }
}

construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 65 bytes (520 bits) size.
    pub struct H520(65);
}

#[cfg(feature = "ink")]
impl SpreadLayout for H520 {
    const FOOTPRINT: u64 = 65 * <u8 as SpreadLayout>::FOOTPRINT;
    const REQUIRES_DEEP_CLEAN_UP: bool = <u8 as SpreadLayout>::REQUIRES_DEEP_CLEAN_UP;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        let a = array_init::<[u8; 65], _>(|_| <u8 as SpreadLayout>::pull_spread(ptr));
        a.into()
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        for elem in &self.0 {
            <u8 as SpreadLayout>::push_spread(elem, ptr)
        }
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        for elem in &self.0 {
            <u8 as SpreadLayout>::clear_spread(elem, ptr)
        }
    }
}

#[cfg(feature = "ink")]
impl PackedLayout for H520 {
    #[inline]
    fn pull_packed(&mut self, at: &Key) {
        for elem in &mut self.0 {
            <u8 as PackedLayout>::pull_packed(elem, at)
        }
    }

    #[inline]
    fn push_packed(&self, at: &Key) {
        for elem in &self.0 {
            <u8 as PackedLayout>::push_packed(elem, at)
        }
    }

    #[inline]
    fn clear_packed(&self, at: &Key) {
        for elem in &self.0 {
            <u8 as PackedLayout>::clear_packed(elem, at)
        }
    }
}

#[cfg(all(feature = "std", feature = "ink"))]
impl StorageLayout for H520 {
    fn layout(key_ptr: &mut KeyPtr) -> Layout {
        let len: u32 = 33;
        let elem_footprint = <u8 as SpreadLayout>::FOOTPRINT;
        Layout::Array(ArrayLayout::new(
            LayoutKey::from(key_ptr.next_for::<H520>()),
            len,
            elem_footprint,
            <u8 as StorageLayout>::layout(&mut key_ptr.clone()),
        ))
    }
}

#[cfg(all(feature = "std", feature = "scale-info"))]
impl TypeInfo for H520 {
    type Identity = Self;

    fn type_info() -> Type {
        TypeDefArray::new(65, MetaType::new::<u8>()).into()
    }
}

#[cfg(feature = "std")]
mod serde_impls {
    use super::*;

    impl_fixed_hash_serde!(H32, 4);
    impl_fixed_hash_serde!(H256, 32);
    impl_fixed_hash_serde!(H264, 33);
    impl_fixed_hash_serde!(H520, 65);
}

mod codec_impls {
    use super::*;

    impl_fixed_hash_codec!(H32, 4);
    impl_fixed_hash_codec!(H256, 32);
    impl_fixed_hash_codec!(H264, 33);
    impl_fixed_hash_codec!(H520, 65);
}

use core::sync::atomic::{AtomicUsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicI64, AtomicI32, AtomicI16, AtomicI8, AtomicBool};

pub trait AtomicBaseTypeExtensions<BaseType : Copy> {
    #[inline(always)]
    fn get(&self) -> BaseType {
        *unsafe { (self as *const Self as *const BaseType).as_ref() }.unwrap()
    }
}

impl AtomicBaseTypeExtensions<usize> for AtomicUsize {}
impl AtomicBaseTypeExtensions<u8> for AtomicU8 {}
impl AtomicBaseTypeExtensions<u16> for AtomicU16 {}
impl AtomicBaseTypeExtensions<u32> for AtomicU32 {}
impl AtomicBaseTypeExtensions<u64> for AtomicU64 {}
impl AtomicBaseTypeExtensions<i8> for AtomicI8 {}
impl AtomicBaseTypeExtensions<i16> for AtomicI16 {}
impl AtomicBaseTypeExtensions<i32> for AtomicI32 {}
impl AtomicBaseTypeExtensions<i64> for AtomicI64 {}
impl AtomicBaseTypeExtensions<bool> for AtomicBool {}

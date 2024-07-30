use core::mem::size_of;

use core::{ptr::slice_from_raw_parts, mem::size_of};

use alloc::{boxed::Box, string::String, format};

pub unsafe fn memzero(dest: *mut u8, len: usize) {
    dest.write_bytes(0, len)
}

/// Copies the bytes in `any` into a new slice of either `size` or, if None, `size_of::<T>()`.
/// This function is unsafe because of possible out-of-bounds access if used improperly.
pub unsafe fn copy_byte_slice_from<T>(any: &T, size: Option<usize>) -> Option<Box<&[u8]>> {
    let ptr = any as *const T as *const u8;
    let slice = slice_from_raw_parts(ptr, size.unwrap_or(size_of::<T>())).as_ref()?;
    Some(Box::new(slice))
}

/// Starting at `seed`, sums up all the bytes in `any` and checks it against `checksum`.
/// This function is unsafe because of possible out-of-bounds access if used improperly.
pub unsafe fn verify_byte_sum_u8_checksum_of<T>(any: &T, size: Option<usize>, seed: u8, checksum: u8) -> Result<(), String> {
    let mut sum = seed;
    let slice = unsafe {
        if let Some(slice) = copy_byte_slice_from(any, size) {
            slice
        } else {
            return Err(String::from("could not copy bytes to slice before calculating sum"))
        }
    };
    for i in 0..slice.len() {
        sum = sum.wrapping_add(slice[i]);
    }

    if sum == checksum {
        Ok(())
    } else {
        Err(format!(
            "invalid checksum: expected {}, got {} (seed is {}, passed size is {:?}, real size is {})",
            checksum, sum, seed, size, size_of::<T>()
        ))
    }
}


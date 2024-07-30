
use core::{ptr::slice_from_raw_parts, mem::{align_of, size_of}};

use alloc::{boxed::Box, vec};
use lib_math::align::align_down;

pub fn ptr_after_end_of<T, R>(value: &T) -> *const R {
    (value as *const T as usize + size_of::<T>()) as *const R
}

pub fn usize_after_end_of<T>(value: &T) -> usize {
    (value as *const T as usize + size_of::<T>()) as usize
}

// [abcdefgh|abcd*fgh|abcdefgh|abcdefgh]
//          ↑    ↑ to be aligned
//          | alignment boundary
// when aligning, this is the result:
// [*fghabcd|efghabcd|efgh----]
//  ↑ this pointer is returned
pub unsafe fn align_pointer_explicit<T>(
    usize: usize,
    size: usize,
    align_to: usize,
) -> Box<T> {
    let usize_deviation = usize % align_to;
    let mut boxed = vec![0_u8; size].into_boxed_slice();
    let orig_bytes = slice_from_raw_parts(align_down(usize, align_to) as *const u8, size + align_to - usize_deviation);
    // SAFETY: We ensure that we only access the bytes we should by starting
    //         from usize_deviation
    let orig_bytes = orig_bytes.as_ref().unwrap();
    boxed.copy_from_slice(&orig_bytes[usize_deviation..(usize_deviation + size)]);

    return Box::from_raw(Box::into_raw(boxed) as *mut T);
}

pub unsafe fn align_pointer<T>(ptr: *const T) -> Box<T> {
    align_pointer_explicit(ptr as usize, size_of::<T>(), align_of::<T>())
}

// Copies the bytes of the referenced memory into the heap,
// making sure the pointer is aligned.
pub fn align_reference<T>(r: &T) -> Box<T> {
    unsafe { align_pointer(r as *const T) }
}


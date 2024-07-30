use core::ptr::null_mut;

use linked_list_allocator::LockedHeap;

const EARLY_HEAP_SIZE: usize = 16 * 1024;

#[repr(align(0x1000))]
#[repr(C)]
pub(crate) struct EarlyHeap {
    pub heap: [u8; EARLY_HEAP_SIZE],
}

impl EarlyHeap {
    pub(crate) const fn new() -> Self {
        Self {
            heap: [0; EARLY_HEAP_SIZE]
        }
    }
}

pub static EARLY_ALLOCATOR: LockedHeap = LockedHeap::empty();
static mut EARLY_HEAP: EarlyHeap = EarlyHeap::new();

pub(crate) fn early_allocator() -> &'static LockedHeap {
    let mut allocator = EARLY_ALLOCATOR.lock();
    if allocator.bottom() == null_mut() {
        // SAFETY: .init is only invoked once, enforced by
        //         the if statement above
        unsafe {
            allocator.init(EARLY_HEAP.heap.as_mut_ptr(), EARLY_HEAP_SIZE);
        }
    }
    &EARLY_ALLOCATOR
}

// For now just keep this useless heap allocator to silence the compiler
#[global_allocator]
static mut USELESS_ALLOCATOR: LockedHeap = LockedHeap::empty();

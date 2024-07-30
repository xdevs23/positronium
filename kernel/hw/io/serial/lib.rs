#![no_std]

use kernel_hw_io::SerialController;

#[cfg_attr(target_arch="x86_64", path="x86_64.rs")]
mod arch;

pub fn create_serial() -> impl SerialController {
    arch::Serial::new_init()
}

pub unsafe fn get_serial() -> impl SerialController {
    unsafe { arch::Serial::new_uninitialized() }
}
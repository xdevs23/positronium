#![no_std]

use kernel_hw_io::SerialController;

#[cfg_attr(target_arch="x86_64", path="x86_64.rs")]
mod arch;

pub fn create_serial() -> ArchSerial {
    arch::Serial::new_init()
}

pub unsafe fn get_serial() -> ArchSerial {
    unsafe { arch::Serial::new_uninitialized() }
}

pub type ArchSerial = arch::Serial;

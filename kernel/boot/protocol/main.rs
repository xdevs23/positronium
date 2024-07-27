#![no_std]
#![no_main]

pub(crate) fn boot_main() -> ! {
    kernel::kernel_main()
}

#[cfg_attr(target_arch="x86_64", path="x86_64/mod.rs")]
pub mod arch;


#![no_std]
#![cfg_attr(target_arch="x86_64", feature(allocator_api))]

#[cfg_attr(target_arch="x86_64", path="x86_64/mod.rs")]
mod arch;

pub trait Cpu {
    fn halt_execution(&self) -> !;
    fn wait_for_interrupt(&self);
}

pub fn create_cpu() -> impl Cpu {
    arch::create_cpu()
}

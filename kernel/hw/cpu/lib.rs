#![no_std]
#![cfg_attr(target_arch="x86_64", feature(allocator_api))]

use core::error::Error;

#[cfg_attr(target_arch="x86_64", path="x86_64/mod.rs")]
mod arch;

pub trait Cpu {
    fn halt_execution(&self) -> !;
    fn wait_for_interrupt(&self);
    fn initialize() -> Result<impl Cpu, impl Error>;
    fn current_cpu() -> Result<impl Cpu, impl Error>;

    /// If supported, executes the pause instruction
    #[inline(always)]
    fn spin_pause() {}
}

pub fn initialize_cpu() -> impl Cpu {
    arch::ArchCpu::initialize().expect("could not initialize CPU")
}

pub fn current_cpu() -> impl Cpu {
    arch::ArchCpu::current_cpu().expect("could not get current CPU")
}

pub type ArchCpu = arch::ArchCpu;

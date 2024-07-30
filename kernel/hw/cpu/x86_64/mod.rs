use core::{arch::x86_64::_mm_pause, error::Error, fmt::{self, Display}};

use gdt::Gdt;
use x86_64::instructions;

use crate::Cpu;

mod gdt;
mod early_alloc;

#[derive(Debug)]
pub struct CpuInitError;
impl Display for CpuInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}
impl Error for CpuInitError {}

// The naming used here is simply because X8664Cpu is super awkward
pub struct Amd64Cpu {}

impl Cpu for Amd64Cpu {
    #[inline(always)]
    fn halt_execution(&self) -> ! {
        instructions::interrupts::disable();
        instructions::hlt();
        unreachable!()
    }

    #[inline(always)]
    fn wait_for_interrupt(&self) {
        instructions::interrupts::enable_and_hlt();
    }

    fn initialize() -> Result<impl Cpu, impl Error> {
        Gdt::init();
        Ok::<_, CpuInitError>(Self {})
    }

    fn current_cpu() -> Result<impl Cpu, impl Error> {
        // TODO: do some CPUID magic or core-local variables or something
        // For now just create a new instance, no strings attached
        Ok::<_, CpuInitError>(Self {})
    }

    #[inline(always)]
    fn spin_pause() {
        unsafe { _mm_pause() }
    }
}

pub(crate) type ArchCpu = Amd64Cpu;

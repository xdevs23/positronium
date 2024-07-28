use x86_64::instructions;

use crate::Cpu;

mod gdt;
mod early_alloc;

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
}

pub(crate) fn create_cpu() -> impl Cpu {
    Amd64Cpu {}
}

extern crate alloc;

use kernel_logging::{CommonLogger, Logger};
use x86_64::registers::segmentation::{Segment, CS, DS, ES, FS, GS, SS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};

use super::early_alloc::early_allocator;

use alloc::boxed::Box;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct Gdt;

impl Gdt {
    pub fn init() {
        let logger = Logger::new("x86_64/gdt/init");

        let allocator = early_allocator();
        let mut gdt = Box::new_in(GlobalDescriptorTable::new(), allocator);

        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector = gdt.append(Descriptor::kernel_data_segment());

        logger.debug("Loading GDT");

        // Leak the GDT so that it lives forever
        Box::leak(gdt).load();

        logger.debug("GDT loaded");

        unsafe {
            CS::set_reg(code_selector);
            DS::set_reg(data_selector);
            ES::set_reg(data_selector);
            FS::set_reg(data_selector);
            GS::set_reg(data_selector);
            SS::set_reg(data_selector);
        }

        logger.debug("All selector registers have been set");
    }
}

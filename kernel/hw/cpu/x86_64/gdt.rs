extern crate alloc;

use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

use super::early_alloc::early_allocator;

use alloc::sync::{Arc};
use alloc::boxed::Box;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct Gdt;

impl Gdt {
    pub fn init() {
        let allocator = early_allocator();
        let mut gdt = Box::new_in(GlobalDescriptorTable::new(), allocator);

        gdt.append(Descriptor::kernel_code_segment());
        gdt.append(Descriptor::user_code_segment());
        gdt.append(Descriptor::user_data_segment());

        // Leak the GDT so that it lives forever
        Box::leak(gdt).load();
    }
}

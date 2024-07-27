#![no_std]

use kernel_hw_cpu::{create_cpu, Cpu};
use kernel_hw_io::SerialController;
use kernel_hw_io_serial::create_serial;

pub fn kernel_main() -> ! {
    let cpu = create_cpu();

    let serial = create_serial();
    _ = serial.write_string("\nPositronium Kernel\n");

    cpu.halt_execution()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    let serial = create_serial();
    _ = serial.write_string("Panic\n");
    create_cpu().halt_execution()
}

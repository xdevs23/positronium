#![no_std]

use kernel_hw_cpu::{current_cpu, initialize_cpu, Cpu};
use kernel_hw_io::SerialController;
use kernel_hw_io_serial::create_serial;

pub fn kernel_main() -> ! {
    let cpu = initialize_cpu();

    let serial = create_serial();
    _ = serial.write_string("\nPositronium Kernel\n");

    cpu.halt_execution()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    let serial = create_serial();
    _ = serial.write_string("Panic\n");
    current_cpu().halt_execution()
}

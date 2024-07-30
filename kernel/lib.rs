#![no_std]

extern crate alloc;

use core::fmt::Write;

use kernel_hw_cpu::{current_cpu, initialize_cpu, Cpu};
use kernel_hw_io_serial::create_serial;
use kernel_logging::{apply_kernel_log_writer, CommonLogger, KernelLogWriter, Logger};

pub fn kernel_main() -> ! {
    apply_kernel_log_writer(KernelLogWriter::new(create_serial()));

    let logger = Logger::new("kernel/main");
    logger.info("Positronium Kernel");

    let cpu = initialize_cpu();

    cpu.halt_execution()
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    let mut serial = create_serial();
    _ = writeln!(&mut serial, "Panic: {}", _info.message().as_str().unwrap_or("<no panic message>"));
    if let Some(loc) = _info.location() {
        _ = writeln!(&mut serial, "Location: {}:{} ({})", loc.file(), loc.line(), loc.column());
    }
    _ = writeln!(&mut serial, "{_info:?}");
    _ = writeln!(&mut serial, "Halting.\n");
    current_cpu().halt_execution()
}

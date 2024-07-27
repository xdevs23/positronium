#![no_std]
#![no_main]

use core::arch::asm;

use kernel_io_defs::SerialController;
use kernel_io_serial::create_serial;
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn native_x86_64_start() -> ! {
    assert!(BASE_REVISION.is_supported());

    let serial = create_serial();
    _ = serial.write_string("\nPositronium Kernel\n");

    halt()
}

fn halt() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    halt();
}

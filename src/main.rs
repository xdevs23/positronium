#![no_std]
#![no_main]

use core::arch::asm;

use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn native_x86_64_start() -> ! {
    assert!(BASE_REVISION.is_supported());
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

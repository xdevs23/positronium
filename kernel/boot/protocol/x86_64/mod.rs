
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
pub unsafe extern "C" fn native_x86_64_start() -> ! {
    assert!(BASE_REVISION.is_supported());
    crate::boot_main()
}


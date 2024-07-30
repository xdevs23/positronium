#![no_std]

pub trait IOPortReadWrite {
    unsafe fn read() -> Result<u8, ()>;
    unsafe fn write(val: u8) -> Result<(), ()>;
}

pub trait IOPortRead {
    unsafe fn read() -> Result<u8, ()>;
}

pub trait IOPortWrite {
    unsafe fn write(val: u8) -> Result<(), ()>;
}

pub trait SerialController {
    fn new_init() -> Self;
    fn init(&self) -> Result<(), ()>;
    fn write_string(&self, str: &str) -> Result<(), ()>;
    unsafe fn new_uninitialized() -> Self;
}

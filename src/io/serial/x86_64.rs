
use core::fmt;

use kernel_io_defs::{IOPortRead, IOPortReadWrite, IOPortWrite, SerialController};
use x86_64::instructions;

const COM1: u16 = 0x3f8; // COM1

struct DataPort;
struct InterruptEnablePort;
struct FIFOControlPort;
struct LineControlPort;
struct ModemControlPort;
struct LineStatePort;

impl IOPortReadWrite for DataPort {
    unsafe fn read() -> Result<u8, ()> {
        Ok(instructions::port::PortRead::read_from_port(COM1))
    }
    unsafe fn write(val: u8) -> Result<(), ()> {
        Ok(instructions::port::PortWrite::write_to_port(COM1, val))
    }
}

impl IOPortWrite for InterruptEnablePort {
    unsafe fn write(val: u8) -> Result<(), ()> {
        Ok(instructions::port::PortWrite::write_to_port(COM1 + 1, val))
    }
}

impl IOPortWrite for FIFOControlPort {
    unsafe fn write(val: u8) -> Result<(), ()> {
        Ok(instructions::port::PortWrite::write_to_port(COM1 + 2, val))
    }
}

impl IOPortWrite for LineControlPort {
    unsafe fn write(val: u8) -> Result<(), ()> {
        Ok(instructions::port::PortWrite::write_to_port(COM1 + 3, val))
    }
}

impl IOPortWrite for ModemControlPort {
    unsafe fn write(val: u8) -> Result<(), ()> {
        Ok(instructions::port::PortWrite::write_to_port(COM1 + 4, val))
    }
}

impl IOPortRead for LineStatePort {
    unsafe fn read() -> Result<u8, ()> {
        Ok(instructions::port::PortRead::read_from_port(COM1 + 5))
    }
}


pub(crate) struct Serial {
    did_init: bool,
}

impl Serial {
    fn is_transmit_empty() -> bool {
        (unsafe { LineStatePort::read().unwrap() & 0x20 }) != 0
    }

    fn has_received() -> bool {
        (unsafe { LineStatePort::read().unwrap() & 1 }) != 0
    }

    fn read() -> Result<u8, ()> {
        while !Self::has_received() {};
        unsafe { DataPort::read() }
    }

    fn write(val: u8) -> Result<(), ()> {
        while !Self::is_transmit_empty() {};
        unsafe { DataPort::write(val) }
    }
}

impl SerialController for Serial {
    fn new_init() -> Self {
        let mut new = Self {
            did_init: false,
        };
        new.did_init = new.init().is_ok();
        new
    }

    fn write_string(&self, str: &str) -> Result<(), ()> {
        for c in str.as_bytes().iter() {
            Self::write(*c)?
        }
        Ok(())
    }

    fn init(&self) -> Result<(), ()> {
        unsafe {
            // Disable interrupts
            InterruptEnablePort::write(0)?;

            // Enable DLAB
            LineControlPort::write(0x80)?;

            // Baud rate of 38400
            DataPort::write(3)?;
            InterruptEnablePort::write(0)?;

            // Disable DLAB and set to 8 bits
            LineControlPort::write(3)?;

            // Enable FIFO, clear queues and set threshold to 14 bytes
            FIFOControlPort::write(0xc7)?;

            // Prepare for takeoff
            ModemControlPort::write(0x0b)?;

            // Test serial. Set to loopback mode.
            ModemControlPort::write(0x1e)?;

            const TEST_BYTE: u8 = 0xae;
            // Send a byte
            DataPort::write(TEST_BYTE)?;
            // Check if byte is the same
            if let Ok(byte) = DataPort::read() {
                if byte != TEST_BYTE {
                    return Err(());
                }
            } else {
                return Err(());
            }

            // Set to normal operation mode
            ModemControlPort::write(0x0f)?;
        }
        Ok(())
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s).or(Err(fmt::Error))
    }
}

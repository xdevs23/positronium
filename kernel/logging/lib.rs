#![no_std]

extern crate alloc;

use core::fmt;

use kernel_hw_io::SerialController;
use kernel_hw_io_serial::ArchSerial;
use spin::RwLock;

type LogLevel = &'static str;

pub struct KernelLogWriter<S : SerialController> {
    serial: S,
}

impl<S : SerialController> KernelLogWriter<S> {
    pub const fn new(serial: S) -> Self {
        Self {
            serial
        }
    }
}

pub const LEVEL_FATAL: LogLevel = "FATAL";
pub const LEVEL_ERROR: LogLevel = "ERROR";
pub const LEVEL_WARN: LogLevel = "WARN";
pub const LEVEL_NOTICE: LogLevel = "NOTICE";
pub const LEVEL_INFO: LogLevel = "INFO";
pub const LEVEL_DEBUG: LogLevel = "DEBUG";
pub const LEVEL_TRACE: LogLevel = "TRACE";

pub trait CommonLogger<TWrite : fmt::Write> {
    fn new(name: &'static str) -> Self;

    fn use_writer<F>(&self, f: F) where F: FnOnce (&mut TWrite);
    fn name(&self) -> &'static str { "" }

    fn msgf(&self, level: LogLevel, args: fmt::Arguments) {
        self.use_writer(|writer| {
            // Ignore errors because we don't want to interfere with the kernel
            let _ = writer.write_fmt(format_args!("{l:<6} {n}: {a}\n", n=self.name(), l=level, a=args));
        })
    }

    fn fatalf(&self, args: fmt::Arguments) { self.msgf(LEVEL_FATAL, args) }
    fn errorf(&self, args: fmt::Arguments) { self.msgf(LEVEL_ERROR, args) }
    fn warnf(&self, args: fmt::Arguments) { self.msgf(LEVEL_WARN, args) }
    fn noticef(&self, args: fmt::Arguments) { self.msgf(LEVEL_NOTICE, args) }
    fn infof(&self, args: fmt::Arguments) { self.msgf(LEVEL_INFO, args) }
    fn debugf(&self, args: fmt::Arguments) { self.msgf(LEVEL_DEBUG, args) }
    fn tracef(&self, args: fmt::Arguments) { self.msgf(LEVEL_TRACE, args) }
    fn fatal(&self, msg: &str) { self.fatalf(format_args!("{}", msg)) }
    fn error(&self, msg: &str) { self.errorf(format_args!("{}", msg)) }
    fn warn(&self, msg: &str) { self.warnf(format_args!("{}", msg)) }
    fn notice(&self, msg: &str) { self.noticef(format_args!("{}", msg)) }
    fn info(&self, msg: &str) { self.infof(format_args!("{}", msg)) }
    fn debug(&self, msg: &str) { self.debugf(format_args!("{}", msg)) }
    fn trace(&self, msg: &str) { self.tracef(format_args!("{}", msg)) }
}

pub(crate) static SHARED_KERNEL_LOG_WRITER: RwLock<Option<KernelLogWriter<ArchSerial>>> = RwLock::new(None);

pub struct Logger {
    name: &'static str,
    log_writer: &'static RwLock<Option<KernelLogWriter<ArchSerial>>>,
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logger").field("name", &self.name).finish()
    }
}

impl fmt::Write for KernelLogWriter<ArchSerial> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.serial.write_str(s)
    }
}

impl CommonLogger<KernelLogWriter<ArchSerial>> for Logger {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            log_writer: &SHARED_KERNEL_LOG_WRITER,
        }
    }

    fn use_writer<F>(&self, f: F) where F: FnOnce (&mut KernelLogWriter<ArchSerial>) {
        let mut writer = self.log_writer.write();
        f(&mut writer.as_mut().unwrap());
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

pub fn apply_kernel_log_writer(writer: KernelLogWriter<ArchSerial>) {
    let mut shared_writer = SHARED_KERNEL_LOG_WRITER.write();
    *shared_writer = Some(writer);
}

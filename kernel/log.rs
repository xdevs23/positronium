use core::fmt;

use alloc::sync::Arc;
use kernel_hw_io::SerialController;
use kernel_hw_io_serial::ArchSerial;
use kernel_logging::CommonLogger;
use lib_sync::lateinit::LateInitArc;
use spin::RwLock;

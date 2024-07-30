#![no_std]

use core::{fmt, slice};

const BUF_LEN: usize = 32;

pub fn copy_string_buffered<F: Fn([u8; BUF_LEN], usize) -> ()>(
    s: &str, receiver: F
) -> Result<(), &'static str> {
    let mut buf: [u8; BUF_LEN] = Default::default();
    let mut index = 0;
    for batch in s.as_bytes().chunks(BUF_LEN - 1) {
        let last_index = (BUF_LEN - 1).min(batch.len());
        buf[..last_index].copy_from_slice(batch);
        if let Some(last) = buf.get_mut(last_index) {
            *last = 0;
        } else {
            return Err("failed to get the last index of buffer");
        }
        receiver(buf, index);
        index += last_index;
    }

    Ok(())
}

pub fn copy_string_buffered_mut<F: FnMut([u8; BUF_LEN], usize) -> ()>(
    s: &str, mut receiver: F
) -> Result<(), &'static str> {
    let mut buf: [u8; BUF_LEN] = Default::default();
    let mut index = 0;
    for batch in s.as_bytes().chunks(BUF_LEN - 1) {
        let last_index = (BUF_LEN - 1).min(batch.len());
        buf[..last_index].copy_from_slice(batch);
        if let Some(last) = buf.get_mut(last_index) {
            *last = 0;
        } else {
            return Err("failed to get the last index of buffer");
        }
        receiver(buf, index);
        index += last_index;
    }

    Ok(())
}

pub fn copy_u8_buffered<F: FnMut([u8; BUF_LEN], usize) -> ()>(
    s: &[u8], mut receiver: F
) -> Result<(), &'static str> {
    let mut buf: [u8; BUF_LEN] = Default::default();
    let mut index = 0;
    for batch in s.chunks(BUF_LEN - 1) {
        let last_index = (BUF_LEN - 1).min(batch.len());
        buf[..last_index].copy_from_slice(batch);
        if let Some(last) = buf.get_mut(last_index) {
            *last = 0;
        } else {
            return Err("failed to get the last index of buffer");
        }
        receiver(buf, index);
        index += last_index;
    }

    Ok(())
}

pub struct StrWriter<'a>(pub &'a mut str);

impl<'a> fmt::Write for StrWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let str_slice = unsafe { slice::from_raw_parts_mut(self.0.as_mut_ptr(), self.0.len()) };
        match copy_string_buffered_mut(
            s,
            |buf, index| str_slice[index..index+buf.len()].copy_from_slice(&buf)
        ) {
            Ok(()) => Ok(()),
            Err(_) => Err(fmt::Error)
        }
    }
}

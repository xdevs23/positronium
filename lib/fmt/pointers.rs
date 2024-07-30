use core::fmt::{self, LowerHex};

use num::Integer;

pub struct PtrFormatted<N : Integer + LowerHex>(pub N);

impl<N : Integer + LowerHex> fmt::Display for PtrFormatted<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:#018x}", self.0))
    }
}

impl<N : Integer + LowerHex> fmt::Debug for PtrFormatted<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

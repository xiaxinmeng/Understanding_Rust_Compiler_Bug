 rust
use std::mem::{transmute, size_of};
use std::raw;  // <--

pub fn as_raw_bytes<'a, T>(obj: &'a T) -> &'a [u8] {
    unsafe {
        transmute(raw::Slice{  // <--
            data: transmute(obj),
            len: size_of::<T>()
        })
    }
}

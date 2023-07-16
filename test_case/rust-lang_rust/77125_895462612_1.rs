rust
use core::{ptr, slice};

#[repr(C)]
struct StrWithLen<T: ?Sized> {
    len: u16, // assume all TypeId strings are <64kB
    text: T,
}

pub struct TypeId {
    hash: u64,
    text: &'static StrWithLen<()>, // actually StrWithLen<str>
}

impl PartialEq for TypeId {
    fn eq(&self, rhs: &Self) -> bool {
        if self.hash != rhs.hash {
            false
        } else if ptr::eq(self.text, rhs.text) {
            true
        } else {
            unsafe {
                let lhs_text = slice::from_raw_parts(
                    &self.text.text as *const _ as *const u8,
                    self.text.len as usize,
                );
                let rhs_text = slice::from_raw_parts(
                    &rhs.text.text as *const _ as *const u8,
                    rhs.text.len as usize,
                );
                lhs_text == rhs_text
            }
        }
    }
}

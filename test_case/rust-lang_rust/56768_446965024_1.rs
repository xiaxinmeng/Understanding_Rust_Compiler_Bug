rust
#![feature(allow_internal_unsafe)]

#[allow_internal_unsafe]
macro_rules! contains_unsafe_code {
    () => {
        unsafe { *(0 as *mut u8) = 42; }
    }
}

pub mod safe {
    pub fn foo() {
        contains_unsafe_code!();
    }
}

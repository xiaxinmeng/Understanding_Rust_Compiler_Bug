rust
fn contains_unsafe_code() {
    unsafe { *(0 as *mut u8) = 42; }
}

#[forbid(unsafe_code)]
pub mod safe {
    pub fn foo() {
        crate::contains_unsafe_code();
    }
}

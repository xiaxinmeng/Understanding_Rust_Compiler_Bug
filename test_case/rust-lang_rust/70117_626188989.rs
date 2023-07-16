rust
#![no_std]

#[no_mangle]
pub extern "C" fn rust_eh_personality() {
}

#[no_mangle]
pub fn zzz(a: Option<usize>) -> usize {
    a.unwrap()
}

rust
#![feature(thread_local)]

#[thread_local]
static A: u8 = 0;

pub fn get_a_ref() -> *const u8 {
    &A
}

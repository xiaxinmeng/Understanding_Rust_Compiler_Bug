 rust
#![crate_name = "asan"]
#![cfg(sanitize = "address")] // crate is a noop without address sanitizer

extern crate alloc_system; // this only works if we use the system allocator

#[link(name = "rustc_asan", kind = "static")] // runtime support library
#[link(name = "pthread")] // native deps
#[link(name = "c")]
extern {}

// Any Rust-provided support necessary for this as well
#[no_mangle]
pub extern fn asan_runtime_foo() {}

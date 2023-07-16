rust
#[no_mangle]
pub fn bar(b: bool) {
    // ...
}

pub fn foo() {
    extern "Rust" {
        fn bar();
    }
    unsafe { bar(); }
}

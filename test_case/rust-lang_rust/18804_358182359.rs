rust
// lib.rs
#![feature(linkage)]

pub fn foo<T>() -> *const() {
    extern {
        #[linkage = "extern_weak"]
        static FOO: *const();
    }
    unsafe { FOO }
}

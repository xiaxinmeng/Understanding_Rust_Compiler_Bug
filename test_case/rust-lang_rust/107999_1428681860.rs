rust
#![crate_type = "lib"]

pub fn generic<T>() {
    unsafe {
        let _v: &T = std::mem::uninitialized();
    }
}

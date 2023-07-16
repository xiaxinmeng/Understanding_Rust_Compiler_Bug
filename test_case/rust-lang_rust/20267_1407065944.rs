rust
// Dynamic library implementation
// If a different Rust crate adds this crate as a dependency, only `rlib` will be compiled
// and the following won't be included.
// However, if specifically building the `cdylib` type with `cargo build`, it will be included
#[cfg(crate_type = "cdylib")]
#[no_mangle]
extern "C" fn example() -> interface::MyGuard {
    interface::MyGuard {
        inner: 123,
    }
}

// The public API of the dynamic library, that is shared both by the dynamic library itself,
// and binary that uses it. This makes it as simple as adding a dependency to your Cargo.toml
// to get access to the API, without separating the dynamic library into two crates "impl" and
// "interface" or using features.
pub mod interface {
    #[repr(C)]
    pub struct MyGuard {
        pub(crate) inner: usize,
    }
    
    impl Drop for MyGuard {
        fn drop(&mut self) {
            println!("guard dropped: {}", self.inner);
        }
    }
}

 rust
extern "rust-intrinsic" {
    pub extern fn volatile_load<T>(src: *T) -> T;
    pub extern fn volatile_store<T>(src: *mut T, value: T);
}

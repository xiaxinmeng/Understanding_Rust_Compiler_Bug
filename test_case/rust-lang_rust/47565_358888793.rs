rust
#![crate_type="lib"]
mod private {
    #[no_mangle]
    pub fn public() {}
}

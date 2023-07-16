 rust
#![crate_type="rlib"]
extern {
    fn malloc(len: usize) -> *mut u8;
}

#[export_name="malloc"]
pub fn _malloc(len: usize) {}

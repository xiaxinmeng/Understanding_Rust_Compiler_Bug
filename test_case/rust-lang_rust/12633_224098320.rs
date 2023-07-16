 rust
pub fn main() {
    unsafe {
        let copy: unsafe extern "rust-intrinsic" fn(*const i32, *mut i32, usize) = std::intrinsics::copy;
        let assign: unsafe fn(*const i32, *mut i32, usize) = std::mem::transmute(copy);
    }
}

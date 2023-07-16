rust
pub unsafe fn foo() -> fn(*const u8) -> u8 {
    fn helper(ptr: *const u8) -> u8 {
        *ptr
    }
    helper
}

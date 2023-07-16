rust
#[no_mangle]
pub extern "C" fn foo(a: &i8, b: &i8) -> i32 {
    (*a + *b) as _
}

rust
#[no_mangle]
pub extern fn foo(a: f64) -> bool {
    a as u64 != 0
}

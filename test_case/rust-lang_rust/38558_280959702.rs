rust
#[no_mangle]
pub extern fn f3(x: &mut [i32]) {
    for item in x {
        *item |= 10;
    }
}

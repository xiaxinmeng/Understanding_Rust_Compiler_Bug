 rust
#[no_mangle]
pub extern "aapcs" fn foo(x: f64) -> f64 {
    x
}

#[no_mangle]
pub extern "C" fn bar(x: f64) -> f64 {
    x
}

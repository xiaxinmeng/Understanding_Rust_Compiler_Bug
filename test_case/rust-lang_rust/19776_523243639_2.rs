rust
// lib.rs
#[no_mangle]
pub extern "C" fn print_things() {
    println!("Hello, world!");
}

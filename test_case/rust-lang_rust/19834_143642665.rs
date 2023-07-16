 rust
#[no_mangle]
pub extern fn print(s: String) {
    println!("{}", s);
}

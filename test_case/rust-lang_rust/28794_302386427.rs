rust
#[no_mangle]
pub extern "system" fn test_fn() -> i32 {
    // Removing this line prevents the segfault.
    // I've tried flushing stdout as well but it doesn't change anything
    println!("In library!");
    123456
}

rust
#[no_mangle]
pub extern "C" fn foo() {
    println!("before read");
    println!("read: {:?}", std::fs::read_to_string("foo.txt"));
    println!("after read");
}

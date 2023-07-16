rust
fn main() {
    const NULL: *mut i32 = std::ptr::null_mut();
    println!("{:?}", NULL);
}

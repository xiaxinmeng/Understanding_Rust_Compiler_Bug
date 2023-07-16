rust
fn main() {
    let a: *const [u8] = b"foo";
    println!("{:04x?}", unsafe { &*(a as *const [u16]) })
} 

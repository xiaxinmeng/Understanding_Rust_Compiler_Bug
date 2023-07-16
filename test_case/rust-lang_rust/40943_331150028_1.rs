rust
#![feature(offset_to)]

fn main() {
    let a = [0u8;10];
    
    println!("{:?}", a[5..].as_ptr().offset_to(a.as_ptr())); // -5
}

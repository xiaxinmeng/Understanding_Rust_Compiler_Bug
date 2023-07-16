Rust
fn main() {
    use std::mem;
    
        const MAGIC: i32 = unsafe {
            mem::transmute([1u8, 2, 3, 4])
        };
        println!("{}", MAGIC);
}

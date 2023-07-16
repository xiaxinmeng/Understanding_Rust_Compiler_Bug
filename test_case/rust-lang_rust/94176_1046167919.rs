Rust
pub fn test(a: Option<u32>) -> Option<u32> {
    if let Some(_) = a {
    } else {
        return None;
    }
    println!("Foo");
}

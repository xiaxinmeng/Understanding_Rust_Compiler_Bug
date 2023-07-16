Rust
fn main() {
    let x = &*Box::new(0);
    let y = &*Box::new(0);
    
    if x as *const _ == y as *const _ {
        println!("Equal!");
    }
}

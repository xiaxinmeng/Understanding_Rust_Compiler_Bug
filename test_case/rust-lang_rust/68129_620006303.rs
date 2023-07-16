rust
fn main() {
    let a = 1u16;
    let b = 0u16;
    
    let a_borrowed = &a;
    
    if a_borrowed > b { // expected `&u16`, found `u16`
        println!("hello");
    }
}

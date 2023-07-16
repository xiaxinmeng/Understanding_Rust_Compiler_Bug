rust
fn main() {
    let mut a = 5;
    
    let closure = || a;
    
    a += 1;
    
    closure();
}

 Rust
fn main() {
    let b = [0,1,2];
    let a : &[u8];
    let c : &[u8; 3] = &b;
    a = c; // coercion here
    println!("{:?}", a);
}

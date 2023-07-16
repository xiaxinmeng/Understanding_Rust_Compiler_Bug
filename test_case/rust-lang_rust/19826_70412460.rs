 Rust
fn main() {
    let a: [u8; 2] = [97, 98];
    //if &a[0..2] == b"ab" {    // works
    if a[0..2] == *b"ab" {    // ICE
        println!("test");
    }
}

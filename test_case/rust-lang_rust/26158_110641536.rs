 rust
#![feature(slice_patterns)]
fn main() {
    let x: &[u32] = &[];
    let [[a, b..]..] = x;
    println!("{:08x}", a);
    for x in b.iter().take(30) {
        println!("{:08x}", x);
    }
}

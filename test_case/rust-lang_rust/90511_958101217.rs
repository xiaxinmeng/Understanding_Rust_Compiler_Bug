rust
fn main() {
    let t = [(); usize::MAX - 0x40];
    println!("Len: {:x}", t.len());
}

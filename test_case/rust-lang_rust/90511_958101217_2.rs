rust
fn main() {
    let t = [(); usize::MAX];
    println!("Len: {:x}", t.len());
}

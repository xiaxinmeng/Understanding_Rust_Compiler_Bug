 Rust
fn main() {
    println!("{}", std::mem::size_of::<[u8, ..(1<<61)]>());
}

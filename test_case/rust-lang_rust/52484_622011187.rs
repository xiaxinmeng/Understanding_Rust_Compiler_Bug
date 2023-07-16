rust
fn main() {
    let a: u8 = 42;
    let b = ((a | 0x0F) - 1) >> 31 > 0;
    println!("{}", b);
}

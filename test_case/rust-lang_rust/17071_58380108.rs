 rust
fn main() {
    static A: u64 = -1 as u8 as u64;
    static B: u64 = A / 2;
    match 1u64 {
        B => { },
        _ => { },
    }
}

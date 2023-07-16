 rust
fn main() {
    match 1u64 {
        0...0xFFFF_FFFF_FFFF_FF => { },
        _ => { },
    }
}

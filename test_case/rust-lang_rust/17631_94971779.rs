 rust
pub fn main() {
    let (a, b) = (1u8, 2u8);

    let _x = match (a, b) {
        (0x1b, b'\\') => 1u8,
        (_, 0x05) => 2u8,
        _ => 5u8,
    };
}

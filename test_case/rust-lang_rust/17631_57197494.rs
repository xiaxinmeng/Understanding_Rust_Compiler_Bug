 rust
pub fn main() {
    let (a, b) = (1u8, 2u8);

    let _x = match (a, b) {
        (0x1b, 0x01) => 1u,
        (_, 0x05u8) => 2u,
        _ => 5u
    };
}

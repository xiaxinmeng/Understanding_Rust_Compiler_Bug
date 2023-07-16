rust
#[repr(u8)]
enum WithWraparoundInvalidValues {
    X = 1,
    Y = 254,
}

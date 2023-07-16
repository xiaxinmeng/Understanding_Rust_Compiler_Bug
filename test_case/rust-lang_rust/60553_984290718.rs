rust
#[repr(u8)]
enum WithFields {
    Tuple(i32) = 1,
    Struct{f: i32} = 3,
    Unit = 5,
}

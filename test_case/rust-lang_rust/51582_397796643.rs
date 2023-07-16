rust
pub fn roundtrip(x: i8) -> i8 {
    (unsafe { std::mem::transmute::<i8, Enum>(x) }) as i8
}

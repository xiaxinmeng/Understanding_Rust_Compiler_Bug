rust
pub struct S(u64, [u8; 8]);

pub fn make(x: u64) -> S {
    S(x, [1, 0, 0, 0, 0, 0, 0, 0])
}

pub fn make_constant() -> S {
    S(5, [0, 0, 0, 0, 1, 0, 0, 0])
}

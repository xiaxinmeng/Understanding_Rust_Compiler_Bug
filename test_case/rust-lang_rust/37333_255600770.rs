 rust
#[inline(never)]
pub fn five() -> i32 {
    5
}

pub fn caller() -> i32 {
    five() + 1
}

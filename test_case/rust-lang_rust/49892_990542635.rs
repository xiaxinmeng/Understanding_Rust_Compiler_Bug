rust
#[derive(Eq, PartialEq)]
pub enum A {
    A(u8),
    B(u8),
    C(u8),
    D(u8),
    E(u8)
}

pub fn cmp(a: A, b: A) -> bool {
    a == b
}

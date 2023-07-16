rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    A = 1,
    B = 7,
}

impl Type {
    fn encode(&self) -> Type {
        match self {
            Type::A => Type::B,
            _ => *self,
        }
    }
}

fn main() {
    assert_eq!(Type::A.encode(), Type::B);
}

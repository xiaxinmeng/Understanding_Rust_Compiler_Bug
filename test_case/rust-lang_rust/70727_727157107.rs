
use std::fmt;

#[derive(Copy, Clone)]
pub enum MyEnum {
    A,
    B,
}

impl AsRef<str> for MyEnum {
    fn as_ref(&self) -> &str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }
}

impl fmt::Display for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

fn main() {
    println!("{}", MyEnum::A);
}

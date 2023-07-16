rust
#![crate_type="rlib"]

#[repr(i8)]
pub enum Type {
    Type1 = 0,
    Type2 = 1
}

pub extern "C" fn test() -> Type {
    Type::Type1
}

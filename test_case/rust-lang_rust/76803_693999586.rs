rust
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Type {
    A = 1,
    B = 7,
}


pub fn encode(v: &Type) -> Type {
    match v {
        Type::A => Type::B,
        _ => *v,
    }
}

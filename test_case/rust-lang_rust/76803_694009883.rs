rust
pub enum Type {
    A,
    B,
}


pub fn encode(v: Type) -> Type {
    match v {
        Type::A => Type::B,
        _ => v,
    }
}

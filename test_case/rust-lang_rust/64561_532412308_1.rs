rust
pub enum Extern {
    None,
    Implicit,
    Explicit(Abi),
}

pub struct BareFnTy {
    pub ext: Extern,
    // ...
}
pub struct ForeignMod {
    pub abi: Option<Abi>,
    // ...
}
pub struct FnHeader {
    pub ext: Extern,
    ...
}

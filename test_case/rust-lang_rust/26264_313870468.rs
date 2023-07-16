Rust
struct S(u32);
impl S {
    fn foo(&self) -> Self {
        let make_self = Self;
        make_self(Self.0);  
    }
}

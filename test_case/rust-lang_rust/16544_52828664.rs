 rust
pub trait Show {
    // ...
    fn size_hint(val: &Self) -> Option<uint> { None }
}

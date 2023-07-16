 rust
macro_rules! wrap {
    ($name:ident) => { struct concat_idents!!(Wrapped, $name)($name); }
}

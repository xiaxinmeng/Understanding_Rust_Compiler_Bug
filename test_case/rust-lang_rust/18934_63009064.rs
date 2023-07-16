 rust
macro_rules! t (
    ($ty:ty) => {{
        let v: $ty = Default::default();
        assert!(v.is_empty());
    }}
);

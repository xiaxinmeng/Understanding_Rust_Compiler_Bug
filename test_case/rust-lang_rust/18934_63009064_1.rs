 rust
macro_rules! t (
    ($ty:ty) => {{
        assert!(default::<$ty>().is_empty());
    }}
);

rust
// Compiles successfully.
pub struct Struct<T>(T)
where
    T: Eq;

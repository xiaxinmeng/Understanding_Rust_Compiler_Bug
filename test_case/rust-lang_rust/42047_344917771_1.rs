rust
pub struct Newtype<T>(T);
pub fn foo() -> Newtype<u64> {
    let mut x = 0;
    drop(&mut x);
    // Does not currently propagate the destination because of the field projection.
    Newtype(x)
}

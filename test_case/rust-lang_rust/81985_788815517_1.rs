rust
struct A<const N: u8>;
impl<N> A<{ N }> {}

 rust
impl<'a,'b> Add<&'b BigInt, BigInt> for &'a BigInt {
    fn add(self, other: &'b BigInt) -> BigInt { ... }
}

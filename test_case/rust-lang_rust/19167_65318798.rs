 rust
impl<Rhs> PartialEq<Rhs> for String where Rhs: Deref<str> { ... }
impl<Lhs> PartialEq<String> for Lhs where Lhs: Deref<str> { ... }

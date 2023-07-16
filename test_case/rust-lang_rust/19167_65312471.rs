 rust
impl<Rhs> PartialEq<Rhs> for String where Rhs: Deref<str> { ... }
impl<'a> PartialEq<String> for &'a str { ... }

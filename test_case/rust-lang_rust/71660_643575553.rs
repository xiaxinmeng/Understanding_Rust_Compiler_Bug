rust
impl PartialEq<Vec<B>> for &[A] where A: PartialEq<B> {...}
impl PartialEq<Vec<B>> for &mut [A] where A: PartialEq<B> {...}

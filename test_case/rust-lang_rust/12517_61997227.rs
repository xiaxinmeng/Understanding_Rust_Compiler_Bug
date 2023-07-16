 rust
impl<T: Eq> PartialEq for T {  //~ error: conflicting implementations for trait `PartialEq`
    fn eq(&self, other: &T) -> bool { Eq::eq(self, other) }
    fn ne(&self, other: &T) -> bool { !Eq::eq(self, other) }
}

impl<T: Eq> Eq for [T, ..2] {
    fn eq(&self, other: &[T, ..2]) -> bool { /* */ }
}

impl<T: PartialEq> PartialEq for [T, ..2] {  // note: note conflicting implementation here
    fn eq(&self, other: &[T, ..2]) -> bool { /* */ }
}

rust
impl IntoIterator for Foo { 
    type IntoIter = impl Iterator<Item = Self::Item>;

    #{defines(Self::IntoIter)}
    fn into_iter(self) -> Self::IntoIter {
        ...
    }
}

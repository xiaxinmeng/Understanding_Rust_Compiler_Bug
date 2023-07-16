 rust
impl<V> Foo<[u8; 20], V> for  BTreeMap<[u8; 20], V> {
    fn foo<'a>(&self, key: &'a [u8; 20]) {
        let range: Range<[u8; 20], V> = Self::range(self, Unbounded: Bound<&'a [u8; 20]>, Included(key));
    }
}

 rust
#[automatically_derived]
impl Eq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        fn eq<T: Eq>(t1: &T, t2: &T) -> bool { t1.eq(t2) }
        eq(&self.field1, &other.field1) && ...
    }
}

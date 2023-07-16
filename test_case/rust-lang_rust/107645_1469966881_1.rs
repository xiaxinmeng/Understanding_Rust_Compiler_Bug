rust
impl Foo for Bar {
    type IntoIter = impl Baz; // defined by methods in this impl (which return this assoc type)
}

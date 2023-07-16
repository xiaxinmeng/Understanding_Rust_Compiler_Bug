rust
impl<'a, 'b> Foo for &'b mut Quix<'a> {
    type Ty = impl Printy + Captures<'a> + Captures<'b>;

    fn bar(self) -> Self::Ty {
        self
    }
}

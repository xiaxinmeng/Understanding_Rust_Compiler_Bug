
impl<'self> Foo {
    fn bleh(&'self self) -> &'self int {
        return &self.x
    }
}

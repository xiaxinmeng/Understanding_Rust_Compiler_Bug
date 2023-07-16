rust
impl<A, B, C> Debug for Foo<A, B, C>
where ?????
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Foo")
            .field("stuff", &self.stuff)
            .field("thing", &self.thing)
            .field("marker", &self.marker)
            .finish()
    }
}

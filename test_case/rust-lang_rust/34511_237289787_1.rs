 rust
pub struct FooReturn<'a, 'b> {
    field: XXX // for some suitable type XXX
}

impl<'a,'b> Iterator for FooReturn<'a,'b> {
    type Item = <XXX as Iterator>::Item;
}

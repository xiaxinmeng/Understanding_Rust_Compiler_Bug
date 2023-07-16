rust
pub trait Trait<'a, 'b> {
    fn method(self, _: &'static &'static ())
    where
        'a: 'b;
}

impl<'a> Trait<'a, 'static> for () {
    fn method(self, _: &'static &'a ()) {}
}

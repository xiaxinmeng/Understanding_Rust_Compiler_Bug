rust
pub trait MapAccess {
    type Error;
    fn next_key_seed(&mut self) -> Option<Self::Error>;
}

struct Access<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> MapAccess for Access<'a> {
    type Error = ();
    fn next_key_seed(&mut self) -> Option<Self::Error> {
        unimplemented!()
    }
}

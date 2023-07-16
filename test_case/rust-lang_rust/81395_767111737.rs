rust
pub struct Generic<T>(T);

impl<'a> std::ops::Deref for Generic<&'a mut ()> {
    type Target = Generic<&'a ()>;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl<'a> Generic<&'a ()> {
    pub fn some_method(&self) {}
}
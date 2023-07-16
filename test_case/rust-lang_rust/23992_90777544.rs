
pub struct Outer<T: Trait>(T);
pub struct Inner<'a>(&'a bool);

pub trait Trait {
    type Error;
    fn ready(self) -> Self::Error;
}

impl<'a> Trait for Inner<'a> {
    type Error = Outer<Inner<'a>>;
    fn ready(mut self) -> Outer<Inner<'a>> {}
}

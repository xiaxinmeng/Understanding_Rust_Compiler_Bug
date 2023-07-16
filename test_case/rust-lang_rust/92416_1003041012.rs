rust
use std::iter::FromIterator;

impl Trait for Foo<u32> {
    fn eval(self) {
        self.map(|v| v);
    }
}


pub(crate) trait Trait {
    fn eval(self);
}


pub struct Foo<T>(T);


impl<T> Foo<T> {
    pub(crate) fn new() -> Self {
        loop {}
    }

    pub fn map(self, f: impl FnMut(T) -> T) {
        let _ = self.into_iter()
            .map(f)
            .collect::<Foo<T>>();
    }
}


impl<T> IntoIterator for Foo<T> {
    type Item = T;
    type IntoIter = Box<dyn Iterator<Item=T>>;

    fn into_iter(self) -> Self::IntoIter {
        loop {}
    }
}


impl<T> FromIterator<T> for Foo<T> {
    fn from_iter<I: IntoIterator<Item=T>>(_it: I) -> Self {
        Foo::<T>::new().map(|val| val);
        Foo::new()
    }
}

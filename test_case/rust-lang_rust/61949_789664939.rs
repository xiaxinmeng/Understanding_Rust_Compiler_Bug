rust
pub struct Foo<'a>(&'a ());

impl<'a> Foo<'a> {
    pub fn works<T>(self) -> impl FnOnce(T) -> Foo<'a> {
        move |_| todo!()
    }
    pub fn errors<T>(self) -> impl FnOnce(T) -> Self {
        move |_| todo!()
    }
}

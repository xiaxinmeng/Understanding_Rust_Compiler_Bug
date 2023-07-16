rust

impl<'a> Yokeable<'a> for Box<dyn IsCovariant<'static> + 'static> {
    type Output = Box<dyn IsCovariant<'a> + 'a>;
}

// this impl is mostly an example and unnecessary for the pure repro
use std::borrow::*;
impl<'a, T: ToOwned + ?Sized> Yokeable<'a> for Cow<'static, T> {
    type Output = Cow<'a, T>;
}
impl<'a, T: ToOwned + ?Sized> IsCovariant<'a> for Cow<'a, T> {}

fn upcast_yoke(y: Yoke<Cow<'static, str>>) -> Yoke<Box<dyn IsCovariant<'static> + 'static>> {
    upcast(y)
}

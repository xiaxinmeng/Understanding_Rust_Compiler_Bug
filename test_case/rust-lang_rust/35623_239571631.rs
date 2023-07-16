 rust
#![feature(specialization, optin_builtin_traits)]
trait MyFrom<T> {
    fn from(_: T) -> Self;
}

impl<T> MyFrom<T> for T {
    default fn from(thing: T) -> T {
        thing
    }
}

trait NotUnit {}
impl NotUnit for .. {}
impl !NotUnit for () {}

impl<T: NotUnit> MyFrom<T> for () {
    fn from(_: T) -> () {
        ()
    }
}

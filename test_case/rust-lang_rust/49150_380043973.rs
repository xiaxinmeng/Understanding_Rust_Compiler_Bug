rust
trait OptionAsPin<T> {
    fn as_pin<'a>(self: Pin<'a, Self>) -> Option<Pin<'a, T>>;
}

impl<T> OptionAsPin<T> for Option<T> {
    fn as_pin<'a>(self: Pin<'a, Self>) -> Option<Pin<'a, T>> {
        match *unsafe { Pin::get_mut(&mut self) } {
            Some(ref mut item) => Some(unsafe { Pin::new_unchecked(item) }),
            None => None,
        }
    }
}

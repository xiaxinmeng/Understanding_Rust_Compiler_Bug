 rust
impl<'b: 'c, 'c, T: ?Sized> Ref<'b, T> {
    pub fn generalized_map<U: ?Sized, F, R>(orig: Ref<'b, T>, f: F) -> R
    where F: FnOnce(&T, &FnOnce(&'c U) -> Ref<'c, U>) -> R {
        f(orig._value, &move |new| Ref {
            _value: new,
            _borrow: orig._borrow,
       })
    }
}

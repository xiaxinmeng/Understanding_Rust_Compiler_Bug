rust
pub struct YoloCell<T: ?Sized> /* still not a serious name proposal */ {
    pub value: T,
    _private: (),  // prevent construction by struct literal
}

impl<T: ?Sized> Copy for YoloCell<T> {}
unsafe impl<T: ?Sized> Sync for YoloCell<T> {}

impl<T> YoloCell<T> {
    pub unsafe const fn new(value: T) -> Self {
        Self { value, _private: () }
    }
}

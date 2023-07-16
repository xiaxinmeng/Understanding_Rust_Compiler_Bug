rust
extern {
    type Private;
}

pub struct RawWaker {
    data: NonNull<Private>,
    vtable: &'static RawWakerVTable<Private>,
}

pub struct RawWakerVTable<T: ?Sized = ()> {
    clone: for<'a> fn(&'a T) -> RawWaker,
    wake: unsafe fn(NonNull<T>),
    wake_by_ref: for<'a> fn(&'a T),
    drop: unsafe fn(NonNull<T>),
}

impl RawWaker {
    pub fn new<T>(data: NonNull<T>, vtable: &'static RawWakerVTable<T>) -> Self {
        ...
    }
}

impl<T> RawWakerVTable<T> {
    #[rustc_promotable]
    #[rustc_allow_const_fn_ptr]
    pub const fn new(
        clone: for<'a> fn(&'a T) -> RawWaker,
        wake: unsafe fn(NonNull<T>),
        wake_by_ref: for<'a> fn(&'a T),
        drop: unsafe fn(NonNull<T>),
    ) -> Self {
        ...
    }
}

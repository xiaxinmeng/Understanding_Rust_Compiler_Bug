rust
pub trait CoerceSized<T> where T: CoerceUnsized<Self> {..}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceSized<Rc<T>> for Rc<U> {}

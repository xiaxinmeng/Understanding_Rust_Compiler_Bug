rust
struct MyBox<T>(*mut T);

impl<T> MyBox<T> {
    fn get(&self) -> Option<&T> {
        match self.0 {
            Self::NULL => None,
            ptr => unsafe { Some(&*ptr) },
        }
    }

    const NULL: *mut T = ::std::ptr::null_mut();
}

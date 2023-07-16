rust
struct MyBox<T>(*mut T);

impl<T> MyBox<T> {
    const NULL: *mut T = ::std::ptr::null_mut();
}

fn get<'a, T>(ptr: *mut T) -> Option<&'a T> {
    match ptr {
        MyBox::<T>::NULL => None,
        ptr => unsafe { Some(&*ptr) },
    }
}

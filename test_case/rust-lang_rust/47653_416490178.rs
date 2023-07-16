rust
unsafe trait Cast<T: ?Sized> {
    fn cast(ptr: *mut Self) -> *mut T;
}

// implied: T: Sized
unsafe impl<T, U: ?Sized> Cast<T> for U {
    fn cast(ptr: *mut Self) -> *mut T {
        ptr as _
    }
}

unsafe impl<T, U> Cast<[T]> for [U] {
    fn cast(slice: *mut Self) -> *mut [T] {
        unsafe {
            let ptr = (*slice).as_mut_ptr();
            let len = (*slice).len();
            std::slice::from_raw_parts_mut(ptr as _, len)
        }
    }
}
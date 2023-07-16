rust
pub fn filter_map<F>(self, f: F) -> Option<P<T>> where
    F: FnOnce(T) -> Option<T>,
{
    unsafe {
        // Transmute to `Box<MaybeUninit<T>>` so if `f` `panic`s or returns
        // `None` the `Box` will be freed but not the inner T.
        let mut maybe_box: Box<MaybeUninit<T>> = mem::transmute(self.ptr);
        let x = f(ptr::read(maybe_box.as_ptr()))?;
        maybe_box.set(x);
        Some(P { ptr: mem::transmute(maybe_box) })
    }
}

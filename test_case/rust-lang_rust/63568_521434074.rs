rust
impl<T> MaybeUninit<T> {
    /// # Safety
    ///
    ///   - the contents must have been initialised
    unsafe
    fn assume_init_with_mut<R, F> (mut self: MaybeUninit<T>, f: F) -> R
    where
        F : FnOnce(&mut T) -> R,
    {
        if mem::needs_drop::<T>().not() {
            return f(unsafe { self.get_mut() });
        }
        let mut this = ::scopeguard::guard(self, |mut this| {
            ptr::drop_in_place(this.as_mut_ptr());
        });
        f(unsafe { MaybeUninit::<T>::get_mut(&mut *this) })
    }
}

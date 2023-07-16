rust
pub fn filter_map<F>(self, f: F) -> Option<P<T>> where
    F: FnOnce(T) -> Option<T>,
{
    unsafe {
        // Transmute to `Box<ManuallyDrop<T>>` so if `f` `panic`s or returns
        // `None` the `Box` will be freed but not the inner T.
        let mut manaully_drop_box: Box<ManuallyDrop<T>> = mem::transmute(self.ptr);
        let x = f(ManuallyDrop::take(&mut manaully_drop_box))?;
        *manaully_drop_box = ManuallyDrop::new(x);
        Some(P { ptr: mem::transmute(manaully_drop_box) })
    }
}

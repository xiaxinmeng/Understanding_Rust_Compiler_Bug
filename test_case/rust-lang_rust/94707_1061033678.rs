rust
// this: &mut Arc<T>
let mut arc: Arc<MaybeUninit<T>> = Self::new_uninit();
unsafe {
    // SAFETY: this `Arc` was just created, so we must hold the only reference to it
    let data: &mut MaybeUninit<T> = Arc::get_mut_unchecked(&mut arc);
    // SAFETY: `MaybeUninit<T>` has the same layout as `T`, so we can write a clone
    // of the contents of `this` into it. `write_clone_into_raw` uses `ptr::write`
    // under the hood, which is allowed when initializing uninitialized data.
    (**this).write_clone_into_raw(data.as_mut_ptr());
    // SAFETY: we just wrote a valid value of `T` to `arc`
    // (assuming, of course, that `this` is a valid `Arc<T>`)
    *this = arc.assume_init();
}

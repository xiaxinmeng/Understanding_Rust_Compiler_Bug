rust
/// unsafe {
///     // This `&mut v` does not count as a use of `v`, but lets us write a valid value over the invalid `Box`
///     ptr::write(&mut v as *mut Box<i32>, Box::new(42i32));
/// }
///
/// // Because we've re-established the invariants of the Box before using it, we can now do whatever we want in safe code.
/// assert_eq!(*v, 42);

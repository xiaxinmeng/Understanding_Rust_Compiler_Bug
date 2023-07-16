rust
// It's also possible to write compile-time initialization functions that suit your specific needs
// with "complex" logic taking advantage of various methods you might typically expect to only
// be available at runtime.
const fn build<T: Copy, const N: usize>(x: [T; N]) -> StaticVec<T, N> {
  // StaticVec implements `Drop`, and so can't *directly* be returned from a `const fn` yet (at
  // least specifically if / when first instantiated as a fully-initialized "naked" function-local
  // variable) even when `T` is `Copy`, making the (sound) use of MaybeUninit below necessary to
  // facilitate regular access to the StaticVec we'll be creating.
  let mut mu = MaybeUninit::new(StaticVec::new());
  // Not actually unsafe here: `sv` is just a mutable reference to a normal empty StaticVec.
  let sv = unsafe { mu.assume_init_mut() };
  // `StaticVec::push` is a `const fn`. Note that loops in `const fn` are limited to `while`
  // currently: if that changes, obviously use an iterator-based `for` loop instead.
  let mut i = 0;
  while i < N {
    sv.push(x[i]);
    i += 1;
  }
  // `StaticVec::pop` is also a `const fn`, so we can do this as well...
  while sv.pop().is_some() {}
  // And put everything back in like this...
  sv.insert_from_slice(0, &x);
  // Still not actually unsafe here: we soundly initialized everything that needed it as soon as we
  // called `StaticVec::new()`.
  unsafe { mu.assume_init() }
  // The methods demonstrated above are by no means the only ones for StaticVec that could be used
  // (and might be useful) in a context like this, so do go ahead and peruse the docs for this crate
  // to find out more.
}

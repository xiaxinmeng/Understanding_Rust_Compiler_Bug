plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
  --> /checkout/library/core/src/macros/mod.rs:40:35
   |
36 | / macro_rules! assert_eq {
37 | |     ($left:expr, $right:expr $(,)?) => ({
38 | |         match (&$left, &$right) {
39 | |             (left_val, right_val) => {
40 | |                 if !(*left_val == *right_val) {
   | |                                   ^^^^^^^^^^ expected union `MaybeUninit`, found type parameter `T`
62 | |     });
63 | | }
   | |_- in this expansion of `assert_eq!`
   |
   |
  ::: library/alloc/src/box_storage/tests.rs:86:15
   |
86 |   fn zst_sanity<T>(v: &Box<[MaybeUninit<T>]>) {
   |                 - this type parameter
87 |       assert_eq!(v.capacity(), usize::MAX);
88 |       assert_eq!(v.as_ptr(), core::ptr::Unique::<T>::dangling().as_ptr());
   |
   |
   = note: expected raw pointer `*const MaybeUninit<T>`
              found raw pointer `*mut T`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

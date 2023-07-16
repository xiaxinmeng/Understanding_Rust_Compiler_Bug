text
error: reached the recursion limit while instantiating `Foo::<u32>::map::<[closure@src/lib.rs:43:29: 43:38]>`
  --> src/lib.rs:43:9
   |
43 |         Foo::<T>::new().map(|val| val);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: `Foo::<T>::map` defined here
  --> src/lib.rs:23:5
   |
23 |     pub fn map(self, f: impl FnMut(T) -> T) {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

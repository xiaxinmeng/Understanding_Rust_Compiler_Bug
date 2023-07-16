
error: implementation of `FnMut` is not general enough
 --> src/main.rs:5:5
  |
5 |     (0..3).filter(opaque(|_| true));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnMut` is not general enough
  |
  = note: `impl Fn(&'2 i32)-> bool` must implement `FnMut<(&'1 i32,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnMut<(&'2 i32,)>`, for some specific lifetime `'2`

error[E0308]: mismatched types
 --> src/main.rs:5:5
  |
2 |     fn opaque<T, F: Fn(T) -> bool>(f: F) -> impl Fn(T) -> bool {
  |                                             ------------------
  |                                             |
  |                                             the expected opaque type
  |                                             the found opaque type
...
5 |     (0..3).filter(opaque(|_| true));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected associated type `<impl Fn(&i32)-> bool as FnOnce<(&i32,)>>::Output`
             found associated type `<impl Fn(&i32)-> bool as FnOnce<(&i32,)>>::Output`
note: the lifetime requirement is introduced here

rs
error: implementation of `FnOnce` is not general enough
 --> src/main.rs:7:5
  |
7 |     takes_closure(|_| ());
  |     ^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
  |
  = note: closure with signature `fn(&'2 u32)` must implement `FnOnce<(&'1 u32,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 u32,)>`, for some specific lifetime `'2`

error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:7:5
  |
7 |     takes_closure(|_| ());
  |     ^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected trait `for<'a> FnOnce<(&'a u32,)>`
             found trait `FnOnce<(&u32,)>`
note: this closure does not fulfill the lifetime requirements
 --> src/main.rs:7:19
  |
7 |     takes_closure(|_| ());
  |                   ^^^
note: the lifetime requirement is introduced here
 --> src/main.rs:4:26
  |
4 | fn takes_closure(_: impl Closure) {}
  |                          ^^^^^^^

For more information about this error, try `rustc --explain E0308`.

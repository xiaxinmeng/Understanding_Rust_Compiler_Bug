
error[E0308]: mismatched types
 --> src/main.rs:6:5
  |
6 |     my_function(closure_consumer);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected trait `for<'a> FnMut<(&'a str,)>`
             found trait `FnMut<(&str,)>`
note: this closure does not fulfill the lifetime requirements
 --> src/main.rs:5:28
  |
5 |     let closure_consumer = |data| {};
  |                            ^^^^^^
note: the lifetime requirement is introduced here
 --> src/main.rs:2:31
  |
2 | fn my_function(callback: impl FnMut(&str) + Send + 'static){}
  |                               ^^^^^^^^^^^

error: implementation of `FnOnce` is not general enough
 --> src/main.rs:6:5
  |
6 |     my_function(closure_consumer);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
  |
  = note: closure with signature `fn(&'2 str)` must implement `FnOnce<(&'1 str,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 str,)>`, for some specific lifetime `'2`

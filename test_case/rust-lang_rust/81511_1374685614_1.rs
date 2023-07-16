
error[E0308]: mismatched types
 --> src/main.rs:5:5
  |
5 |     my_function(closure_consumer);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected trait `for<'a, 'b> FnMut<(&'a mut i32, &'b mut i32)>`
             found trait `for<'a> FnMut<(&'a mut i32, &mut i32)>`
note: this closure does not fulfill the lifetime requirements
 --> src/main.rs:4:28
  |
4 |     let closure_consumer = |_a: &mut i32, _b| {};
  |                            ^^^^^^^^^^^^^^^^^^
note: the lifetime requirement is introduced here
 --> src/main.rs:1:32
  |
1 | fn my_function(_callback: impl FnMut(&mut i32, &mut i32)) {}
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
 --> src/main.rs:5:5
  |
5 |     my_function(closure_consumer);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
  |
  = note: expected trait `for<'a, 'b> FnOnce<(&'a mut i32, &'b mut i32)>`
             found trait `for<'a> FnOnce<(&'a mut i32, &mut i32)>`
note: this closure does not fulfill the lifetime requirements
 --> src/main.rs:4:28
  |
4 |     let closure_consumer = |_a: &mut i32, _b| {};
  |                            ^^^^^^^^^^^^^^^^^^
note: the lifetime requirement is introduced here
 --> src/main.rs:1:32
  |
1 | fn my_function(_callback: impl FnMut(&mut i32, &mut i32)) {}
  |                                ^^^^^^^^^^^^^^^^^^^^^^^^^

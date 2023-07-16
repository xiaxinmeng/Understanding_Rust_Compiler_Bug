text
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter 'a in function call due to conflicting requirements
 --> src/lib.rs:6:5
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 5:8...
 --> src/lib.rs:5:8
  |
5 | fn bar<'a>(x: &'a i32) {
  |        ^^
note: ...so that reference does not outlive borrowed content
 --> src/lib.rs:6:9
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |         ^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the reference type `&'static fn(&i32)` does not outlive the data it points at
 --> src/lib.rs:6:5
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0490]: a value of type `fn(&'a i32)` is borrowed for too long
 --> src/lib.rs:6:12
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |            ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: the type is valid for the static lifetime
note: but the borrow lasts for the lifetime 'a as defined on the function body at 5:8
 --> src/lib.rs:5:8
  |
5 | fn bar<'a>(x: &'a i32) {
  |        ^^

error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
 --> src/lib.rs:6:12
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |            ^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 5:8...
 --> src/lib.rs:5:8
  |
5 | fn bar<'a>(x: &'a i32) {
  |        ^^
note: ...so that the type `fn(&'a i32)` is not borrowed for too long
 --> src/lib.rs:6:12
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |            ^^^^^^^^^^^^^^^^^^^^^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that reference does not outlive borrowed content
 --> src/lib.rs:6:12
  |
6 |     foo(x, &(baz as fn(&'a i32)));
  |            ^^^^^^^^^^^^^^^^^^^^^

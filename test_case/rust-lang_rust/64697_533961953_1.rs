text
error[E0490]: a value of type `fn(&'a i32) {foo::<&'a i32>}` is borrowed for too long
 --> src/lib.rs:4:25
  |
4 |     let _: &'static _ = &foo::<&'a i32>;
  |                         ^^^^^^^^^^^^^^^
  |
  = note: the type is valid for the static lifetime
note: but the borrow lasts for the lifetime 'a as defined on the function body at 3:8
 --> src/lib.rs:3:8
  |
3 | fn bar<'a>(_: &'a i32) {
  |        ^^

error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
 --> src/lib.rs:4:25
  |
4 |     let _: &'static _ = &foo::<&'a i32>;
  |                         ^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 3:8...
 --> src/lib.rs:3:8
  |
3 | fn bar<'a>(_: &'a i32) {
  |        ^^
note: ...so that the type `fn(&'a i32) {foo::<&'a i32>}` is not borrowed for too long
 --> src/lib.rs:4:25
  |
4 |     let _: &'static _ = &foo::<&'a i32>;
  |                         ^^^^^^^^^^^^^^^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that reference does not outlive borrowed content
 --> src/lib.rs:4:25
  |
4 |     let _: &'static _ = &foo::<&'a i32>;
  |                         ^^^^^^^^^^^^^^^

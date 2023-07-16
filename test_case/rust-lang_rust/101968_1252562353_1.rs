
error: concrete type differs from previous defining opaque type use
 --> src/lib.rs:6:19
  |
6 |     move |n: i32| async move { n < *number }
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `impl Future<Output = bool>`, got `impl Future<Output = bool>`
  |
note: previous use here
 --> src/lib.rs:6:5
  |
6 |     move |n: i32| async move { n < *number }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0720]: cannot resolve opaque type
 --> src/lib.rs:5:67
  |
5 | fn less_than<'a>(number: &'a i32) -> impl FnOnce<(i32,), Output = impl Future<Output = bool> + 'a> {
  |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot resolve opaque type

For more information about this error, try `rustc --explain E0720`.

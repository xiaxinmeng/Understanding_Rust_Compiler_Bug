
error[E0038]: the trait `Error` cannot be made into an object
 --> src\main.rs:3:37
  |
3 | static_assertions::assert_impl_all!(dyn Error: Error);
  |                                     ^^^^^^^^^ `Error` cannot be made into an object
  |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
 --> src\main.rs:1:14
  |
1 | trait Error: Sized {}
  |       -----  ^^^^^ ...because it requires `Self: Sized`
  |       |
  |       this trait cannot be made into an object...

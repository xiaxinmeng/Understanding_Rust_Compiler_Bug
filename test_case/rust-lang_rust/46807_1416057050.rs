
error[E0038]: the trait `Into` cannot be made into an object
 --> src/lib.rs:3:30
  |
3 | fn named_thread<'s, F>(name: Into<String>, f: F) -> JoinHandle<()>
  |                              ^^^^^^^^^^^^ `Into` cannot be made into an object
  |
  = note: the trait cannot be made into an object because it requires `Self: Sized`
  = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

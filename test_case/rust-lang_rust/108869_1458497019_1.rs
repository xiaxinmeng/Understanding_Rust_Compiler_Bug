
warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
 --> tests/ui/async-await/in-trait/object-safety.rs:3:12
  |
3 | #![feature(async_fn_in_trait)]
  |            ^^^^^^^^^^^^^^^^^
  |
  = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
  = note: `#[warn(incomplete_features)]` on by default

error[E0038]: the trait `Foo` cannot be made into an object
  --> tests/ui/async-await/in-trait/object-safety.rs:11:12
   |
11 |     let x: &dyn Foo = todo!();
   |            ^^^^^^^^ `Foo` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> tests/ui/async-await/in-trait/object-safety.rs:7:14
   |
6  | trait Foo {
   |       --- this trait cannot be made into an object...
7  |     async fn foo(&self);
   |              ^^^ ...because method `foo` is `async`
   = help: consider moving `foo` to another trait

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0038`.

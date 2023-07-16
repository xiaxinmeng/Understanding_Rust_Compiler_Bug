
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> src/main.rs:12:14
   |
11 | pub trait MyTrait: Sync {
   |           ------- this trait cannot be made into an object...
12 |     async fn foo(self: Arc<Self>) {}
   |              ^^^ ...because method `foo` references the `Self` type in its `where` clause
   = help: consider moving `foo` to another trait
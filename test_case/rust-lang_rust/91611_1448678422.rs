
error[[E0038]](https://doc.rust-lang.org/nightly/error_codes/E0038.html): the trait `Trait` cannot be made into an object
 --> src/main.rs:8:13
  |
8 |     let _:  &dyn Trait = todo!();
  |             ^^^^^^^^^^ `Trait` cannot be made into an object
  |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
 --> src/main.rs:4:14
  |
3 | trait Trait {
  |       ----- this trait cannot be made into an object...
4 |     async fn test(&self);
  |              ^^^^ ...because method `test` is `async`
  = help: consider moving `test` to another trait

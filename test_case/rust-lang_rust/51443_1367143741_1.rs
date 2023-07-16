console
warning: the trait `Trait` cannot be made into an object
 --> src/main.rs:2:8
  |
2 |     fn method(&self) where Self: Sync;
  |        ^^^^^^
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
 --> src/main.rs:2:8
  |
1 | pub trait Trait {
  |           ----- this trait cannot be made into an object...
2 |     fn method(&self) where Self: Sync;
  |        ^^^^^^ ...because method `method` references the `Self` type in its `where` clause
  = help: consider moving `method` to another trait
  = note: `#[warn(where_clauses_object_safety)]` on by default

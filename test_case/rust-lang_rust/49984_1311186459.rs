
error[E0782]: trait objects must include the `dyn` keyword
 --> src/lib.rs:2:5
  |
2 |     Eq::eq(&(), &())
  |     ^^
  |
help: add `dyn` keyword before this trait
  |
2 |     <dyn Eq>::eq(&(), &())
  |     ++++   +

error[E0038]: the trait `Eq` cannot be made into an object
 --> src/lib.rs:2:5
  |
2 |     Eq::eq(&(), &())
  |     ^^ `Eq` cannot be made into an object
  |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>)

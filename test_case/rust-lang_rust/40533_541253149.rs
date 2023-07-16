
error[E0038]: the trait `Foo` cannot be made into an object
 --> src/lib.rs:5:19
  |
5 | type BoxFoo = Box<Foo<Bar = u8>>;
  |                   ^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
  |
  = note: the trait cannot use `Self` as a type parameter in the supertraits or where-clauses

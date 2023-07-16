
error[E0038]: the trait `FooAndBar` cannot be made into an object
 --> src/main.rs:7:16
  |
7 |     let x: Box<FooAndBar> = unimplemented!();
  |                ^^^^^^^^^ the trait `FooAndBar` cannot be made into an object
  |
  = note: the trait cannot use `Self` as a type parameter in the supertraits or where-clauses

error: aborting due to previous error

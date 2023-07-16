
error[E0191]: the value of the associated type `Assoc` (from the trait `A`) must be specified
 --> src/lib.rs:8:19
  |
8 | fn takes_b(b: Box<B>) {}
  |                   ^ missing associated type `Assoc` value

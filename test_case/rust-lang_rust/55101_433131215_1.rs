
error[E0191]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified
  --> src/test/run-pass/traits/trait-alias-object-type.rs:24:13
   |
24 |     let c: &dyn I32Iterator = &vec![123].into_iter();
   |             ^^^^^^^^^^^^^^^ missing associated type `Item` value

error: aborting due to previous error

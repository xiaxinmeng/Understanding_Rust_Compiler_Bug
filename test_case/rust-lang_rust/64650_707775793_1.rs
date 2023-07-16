
error[E0308]: mismatched types
  --> src/main.rs:28:14
   |
28 |     take_cfn(Box::new(closure) as Box<dyn CloneableFn>, A);
   |              ^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `for<'r> std::ops::Fn<(&'r A,)>`
              found type `std::ops::Fn<(&A,)>`

text
error[E0277]: the trait bound `u32: Trait` is not satisfied
  --> ....rs:24:3
   |
13 |   <u32 as Trait>::test();
   |   ^^^^^^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `u32`
   |
note: required by `Trait::test`
  --> ....rs:15:3
   |
4  |   fn test();
   |   ^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

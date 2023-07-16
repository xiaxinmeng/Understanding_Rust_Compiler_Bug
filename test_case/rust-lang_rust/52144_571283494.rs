
error[E0423]: expected function, tuple struct or tuple variant, found struct `SomeId`
  --> file17.rs:22:13
   |
22 |         id: SomeId(32),
   |             ^^^^^^
   |             |
   |             constructor is not visible here due to private fields
   |             help: a tuple variant with a similar name exists: `Some`

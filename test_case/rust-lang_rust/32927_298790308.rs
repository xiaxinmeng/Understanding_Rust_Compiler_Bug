
first case, without Sized:

error[E0283]: type annotations required: cannot resolve `_: Trait`
 --> test.rs:6:5
  |
6 |     Trait::associated();
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: required by `Trait::associated`

error: aborting due to previous error

for the second case, with Sized:

error[E0282]: type annotations needed
 --> test.rs:6:5
  |
6 |     Trait::associated();
  |     ^^^^^^^^^^^^^^^^^ cannot infer type for `Self`

error: aborting due to previous error

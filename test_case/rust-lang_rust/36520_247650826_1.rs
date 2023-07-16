 bash
error: expected function, found `Test`
 --> file.rs:7:13
  |
7 |     let x = Test::Variant(1);
  |             ^^^^^^^^^^^^^^^^ empty enum variant called like a function
  |
  = help: did you mean to write: `Test::Variant`?
note: defined here
 --> file.rs:2:5
  |
2 |     Variant,
  |     ^^^^^^^

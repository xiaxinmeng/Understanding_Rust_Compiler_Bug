
error: `Test::Variant` is being called, but it is not a function
 --> test.rs:8:13
  |
8 |     let x = Test::Variant("World");
  |             ^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: did you mean to write `Test::Variant`?
note: defined here
 --> test.rs:2:5
  |
2 |     Variant,
  |     ^^^^^^^

error: aborting due to previous error(s)

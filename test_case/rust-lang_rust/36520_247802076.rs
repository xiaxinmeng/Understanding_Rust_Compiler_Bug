 bash
error[E0423]: `Test::Variant2` is the name of a struct or struct variant, but this expression uses it like a function name
 --> unit.rs:8:13
  |
8 |     let y = Test::Variant2("World");
  |             ^^^^^^^^^^^^^^ struct called like a function
  |
  = help: did you mean to write: `Test::Variant2 { /* fields */ }`?

error: expected function, found unit variant `Test::Variant`
 --> unit.rs:7:13
  |
7 |     let x = Test::Variant("Hello");
  |             ^^^^^^^^^^^^^^^^^^^^^^ unit enum variant called like a function
  |
  = help: did you mean to write: `Test::Variant`?
note: defined here
 --> unit.rs:2:5
  |
2 |     Variant,
  |     ^^^^^^^

error: aborting due to previous error

 bash
error[E0423]: `Test::Variant2` is the name of a struct or struct variant, but this expression uses it like a function name
 --> file.rs:8:13
  |
8 |     let y = Test::Variant2;
  |             ^^^^^^^^^^^^^^ struct called like a function
  |
  = help: did you mean to write: `Test::Variant2 { /* fields */ }`?

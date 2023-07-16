
error: macro definition is not supported in `trait`s or `impl`s
 --> src/main.rs:4:5
  |
4 |     macro_rules! bar {
  |     ^^^^^^^^^^^^^^^^

error: cannot find macro `bar` in this scope
  --> src/main.rs:11:9
   |
11 |         bar!();
   |         ^^^

error: aborting due to 2 previous errors


error: cannot find macro `macroname2` in this scope
 --> src/main.rs:3:5
  |
3 |     macroname2!();
  |     ^^^^^^^^^^ help: a macro with a similar name exists: `macroname`
...
6 | macro_rules! macroname {
  | ---------------------- similarly named macro `macroname` defined here
  |
  = help: have you added the `#[macro_use]` on the module/import?

warning: unused macro definition: `macroname2`
 --> src/main.rs:9:14
  |
9 | macro_rules! macroname2 {
  |              ^^^^^^^^^^
  |
  = note: `#[warn(unused_macros)]` on by default

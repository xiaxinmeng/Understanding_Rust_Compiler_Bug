
warning: enum is never used: `Void`
 --> file2.rs:1:1
  |
1 | enum Void { }
  | ^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `process`
 --> file2.rs:3:1
  |
3 | fn process(input: *const Void) {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

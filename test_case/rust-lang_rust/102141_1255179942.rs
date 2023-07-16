`
error: Undefined Behavior: scalar size mismatch: expected 8 bytes but got 1 bytes instead
 --> src/main.rs:8:13
  |
8 |     let i = 42_u8 as dyn* Debug;
  |             ^^^^^^^^^^^^^^^^^^^ scalar size mismatch: expected 8 bytes but got 1 bytes instead
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
  = note: BACKTRACE:
  = note: inside `main` at src/main.rs:8:13

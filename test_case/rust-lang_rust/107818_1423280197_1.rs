`
error: Undefined Behavior: constructing invalid value: encountered uninitialized memory, but expected an integer
 --> src/main.rs:2:12
  |
2 |     return f(22);
  |            ^^^^^ constructing invalid value: encountered uninitialized memory, but expected an integer
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
  = note: BACKTRACE:
  = note: inside `main` error: Undefined Behavior: constructing invalid value: encountered uninitialized memory, but expected an integer



error: this boolean expression contains a logic bug
 --> src/lib.rs:2:8
  |
2 |     if (x <= 8) && (x > 8) {
  |        ^^^^^^^^^^^^^^^^^^^ help: it would look like the following: `false`
  |
  = note: `#[deny(clippy::logic_bug)]` on by default
help: this expression can be optimized out by applying boolean operations to the outer expression
 --> src/lib.rs:2:8
  |
2 |     if (x <= 8) && (x > 8) {
  |        ^^^^^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#logic_bug

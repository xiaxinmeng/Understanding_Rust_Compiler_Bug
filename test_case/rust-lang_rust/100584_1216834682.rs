
warning: unused variable: `x`
 --> main.rs:1:8
  |
1 | fn foo(x: &str) {
  |        ^ unused variable
2 |        let _ = "{x}";
  |                  - you might have meant to use string interpolation in this string literal
help: string interpolation only works in `format!` invocations
  |
2 |        let _ = format!("{x}");
  |                ++++++++     +
  |
  = note: `#[warn(unused_variables)]` on by default


warning: unused variable: `xyza`
 --> main.rs:1:8
  |
1 | fn foo(xyza: &str) {
  |        ^^^^
  |        |
  |        unused variable
  |        help: if this is intentional, prefix it with an underscore: `_xyza`
2 |     let _ = "{xyza}";
  |             -------- you might have meant to use string interpolation in this string literal
  |
  = note: `#[warn(unused_variables)]` on by default
  = help: string interpolation only works in `format!` invocations

warning: 1 warning emitted


warning: unused variable: `x`
 --> main.rs:1:8
  |
1 | fn foo(x: &str) {
  |        let _ = "{x}";
  |                     ======= x needs to be inside of a format!() invocation
  |
  = note: `#[warn(unused_variables)]` on by default


rust
warning: unary minus has lower precedence than method call
 [--> src/main.rs:2:20
](https://play.rust-lang.org/#)  |
2 |     println!("{}", -3.5_f64.abs());
  |                    ^^^^^^^^^^^^^^ help: consider adding parentheses to clarify your intent: `-(3.5_f64.abs())`
  |
  = note: `#[warn(clippy::precedence)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#precedence

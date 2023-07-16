
fn main() {
    for i in 0 .. 10 {}
}

warning: unused variable: `i`
 --> ...\test.rs:2:9
  |
2 |     for i in 0 .. 10 {}
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by d

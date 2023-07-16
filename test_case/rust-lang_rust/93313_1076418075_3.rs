
warning: unreachable expression
 [--> src/main.rs:6:6
](https://play.rust-lang.org/#)  |
5 |     never_returns();
  |     --------------- any code following this expression is unreachable
6 |     &a; // ERROR
  |      ^ unreachable expression
  |
  = note: `#[warn(unreachable_code)]` on by default

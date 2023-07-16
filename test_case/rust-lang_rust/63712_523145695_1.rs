
error[E0267]: `continue` inside of an `async` closure
 --> src/main.rs:6:9
  |
5 | let _ = async || {
  |         -------- enclosing `async` closure
6 |         continue;
  |         ^^^^^^^^ cannot `continue` inside of an `async` closure

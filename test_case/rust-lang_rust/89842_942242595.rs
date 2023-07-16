
 --> src/main.rs:2:5
  |
2 | /     (
3 | |         println!("Work"),
4 | |         return
  | |         ------ any code following this expression is unreachable
5 | |     ).1
  | |_____^ unreachable expression
  |
  = note: `#[warn(unreachable_code)]` on by default

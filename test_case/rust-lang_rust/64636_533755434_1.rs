
warning: unreachable expression
 --> src/lib.rs:1:16
  |
1 |   async fn foo() {
  |  ________________^
2 | |     loop {
3 | |         return;
4 | |     }
5 | | }
  | |_^
  |
  = note: `#[warn(unreachable_code)]` on by default

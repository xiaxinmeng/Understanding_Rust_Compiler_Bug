
warning: unreachable expression
  --> src/main.rs:18:16
   |
18 |         break && break;
   |                ^^-----
   |                | |
   |                | any code following this expression is unreachable
   |                unreachable expression
   |
   = note: `#[warn(unreachable_code)]` on by default

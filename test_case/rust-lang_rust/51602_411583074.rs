
error: expected `{`, found `in`
 --> src/main.rs:2:10
  |
2 |       if i in 0..10 {
  |       --   -^
  |  _____|____|
  | |     |
  | |     this `if` statement has a condition, but no block
3 | |         break;
4 | |     }
  | |_____- help: try placing this code inside a block: `{ (0..10) <- { break ; }; }`

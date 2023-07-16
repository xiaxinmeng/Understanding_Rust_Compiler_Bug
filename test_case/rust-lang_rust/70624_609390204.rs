
error: expected unsuffixed literal or identifier, found `unix!()`
  --> src/main.rs:3:23
   |
3  |                 #[cfg($config)]
   |                       ^^^^^^^
...
17 |     breakme!(unix!(); "test");
   |     -------------------------- in this macro invocation

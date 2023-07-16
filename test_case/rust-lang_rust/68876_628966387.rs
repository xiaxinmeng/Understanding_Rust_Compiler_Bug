
error[E0608]: cannot index into a value of type `({integer}, {integer})`
 --> src/main.rs:2:3
  |
2 |   dbg!(1, 3)[1];
  |   ^^^^^^^^^^^^^
  |
  = help: to access tuple elements, use tuple indexing syntax (e.g., `tuple.0`)

rust
warning: unreachable block in `if` expression
 --> src/main.rs:2:23
  |
2 |     let x = if{return}{println!("bar");};
  |                       ^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unreachable_code)] on by default

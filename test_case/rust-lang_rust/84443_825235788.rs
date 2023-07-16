rust
  |
5 |     println!("{}", a.x);
  |                      ^ unknown field
  |
help: one of the expressions' fields has a field of the same name
  |
5 |     println!("{}", a.0.x);
  |                      ^^

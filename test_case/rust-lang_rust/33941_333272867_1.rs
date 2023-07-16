
 --> src/main.rs:4:5
  |
4 |     for _ in HashMap::new().iter().cloned() { }
  |     ^^^^^^^^^^^^^^^^^^^^^^^-------^^^^^^^^^^^^^ expected tuple, found reference
  |                             |
  |                             help: did you mean to use `values()` here?

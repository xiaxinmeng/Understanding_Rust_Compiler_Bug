
error[E0425]: cannot find function `bar` in this scope
 --> src/lib.rs:5:9
  |
5 |         bar();
  |         ^^^ not found in this scope
  |
help: consider using the associated function
  |
5 |         Self::bar();
  |         ~~~~~~~~~

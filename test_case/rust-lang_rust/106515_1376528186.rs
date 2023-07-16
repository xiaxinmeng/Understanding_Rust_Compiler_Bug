
error[E0618]: expected function, found `{integer}`
 --> src/main.rs:2:13
  |
2 |       let x = 5
  |  _____________^
3 | |
4 | |     ()
  | |______- call expression requires function
  |
help: consider adding a semicolon here:
  |
2 | let x = 5;
  |          +

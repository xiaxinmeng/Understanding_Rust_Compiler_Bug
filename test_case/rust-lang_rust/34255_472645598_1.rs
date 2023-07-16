
error[E0425]: cannot find value `input_cells` in this scope
 --> src/main.rs:6:9
  |
6 |         input_cells: Vec::new()
  |         ^^^^^^^^^^^ a field by this name exists in `Self`

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
 --> src/main.rs:6:27
  |
6 |         input_cells: Vec::new()
  |                           ^^^^^ only `Fn` traits may use parentheses

error[E0107]: wrong number of type arguments: expected 1, found 0
 --> src/main.rs:6:22
  |
6 |         input_cells: Vec::new()
  |                      ^^^^^^^^^^ expected 1 type argument


error[E0507]: cannot move out of indexed content
 --> e1.rs:3:9
  |
3 | let e = v[0];
  |         ^^^^
  |         |
  |         cannot move out of indexed content
  |         help: consider using a reference instead `&v[0]`

error: aborting due to previous error


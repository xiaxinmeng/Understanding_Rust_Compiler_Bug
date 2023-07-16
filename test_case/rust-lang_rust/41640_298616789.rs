
error[E0507]: cannot move out of indexed content
 --> ex1.rs:4:13
  |
4 |     let e = v[0];
  |             ^^^^
  |             |
  |             help: consider using a reference instead `&v[0]`
  |             cannot move out of indexed content

error: aborting due to previous error

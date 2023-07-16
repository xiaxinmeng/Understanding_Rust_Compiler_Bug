
error[E0507]: cannot move out of indexed content
 --> ex1.rs:4:13
  |
4 |     let e = v[0];
  |             ^^^^
  |             |
  |             consider changing to `&v[0]`
  |             cannot move out of indexed content
  |

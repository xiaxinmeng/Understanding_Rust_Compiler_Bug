
error[E0507]: cannot move out of indexed content
 --> example1.rs:3:18
  |
3 |     let (a, b) = x[0];
  |          -  -    ^^^^ cannot move out of indexed content
  |          |  |
  |          |  ...and here (use `ref b` or `ref mut b`)
  |          hint: to prevent move, use `ref a` or `ref mut a`

error: aborting due to previous error

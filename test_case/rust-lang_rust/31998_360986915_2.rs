
error[E0597]: `what` does not live long enough
 --> src/main.rs:7:1
  |
5 |         func(&mut what);
  |                   ---- borrow occurs here
6 |     }
7 | }
  | ^ `what` dropped here while still borrowed
  |
  = note: values in a scope are dropped in the opposite order they are created

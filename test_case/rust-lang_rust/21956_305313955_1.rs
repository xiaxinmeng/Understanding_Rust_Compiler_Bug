
error[E0499]: cannot borrow `x` as mutable more than once at a time
  --> test.rs:14:5
   |
13 |     x.perform_mut();
   |     - first mutable borrow occurs here
14 |     x.perform_mut();
   |     ^ second mutable borrow occurs here
15 | }
   | - first borrow ends here

error: aborting due to previous error(s)


error[E0499]: cannot borrow `xorcism` as mutable more than once at a time
  --> src\lib.rs:46:5
   |
45 |     xorcism.munge(&[1, 2, 3]);
   |     ------- first mutable borrow occurs here
46 |     xorcism.munge(&[1, 2, 3]);
   |     ^^^^^^^
   |     |
   |     second mutable borrow occurs here
   |     first borrow later used here

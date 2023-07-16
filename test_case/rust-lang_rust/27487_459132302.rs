
error[E0499]: cannot borrow `entry.0` as mutable more than once at a time
  --> src/main.rs:11:26
   |
11 |         let FragmentRepr(ref mut discrs) = *entry;
   |                          ^^^^^^^^^^^^^^ mutable borrow starts here in previous iteration of loop
...
14 | }
   | - mutable borrow ends here

error[E0506]: cannot assign to `entry` because it is borrowed
  --> src/main.rs:12:9
   |
11 |         let FragmentRepr(ref mut discrs) = *entry;
   |                          -------------- borrow of `entry` occurs here
12 |         entry = &mut discrs[0];
   |         ^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `entry` occurs here

error: aborting due to 2 previous errors

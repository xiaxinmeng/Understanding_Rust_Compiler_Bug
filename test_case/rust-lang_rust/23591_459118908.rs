
error[E0502]: cannot borrow `*slice` as mutable because `slice[..]` is also borrowed as immutable
  --> src/main.rs:15:9
   |
14 |     if cmp(&slice[0], &slice[1]) {
   |             -------- immutable borrow occurs here
15 |         slice.swap(0, 1);
   |         ^^^^^ mutable borrow occurs here
16 |     }
17 | }
   | - immutable borrow ends here

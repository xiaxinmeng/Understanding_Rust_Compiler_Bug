
error[E0506]: cannot assign to `*next` because it is borrowed
 --> src/main.rs:6:9
  |
6 |         *next = 22;
  |         ^^^^^^^^^^ assignment to borrowed `*next` occurs here
7 |         cur = next;
  |               ---- borrow of `*next` occurs here

error[E0499]: cannot borrow `*next` as mutable more than once at a time
  --> src/main.rs:7:15
   |
7  |         cur = next;
   |               ^^^^ mutable borrow starts here in previous iteration of loop
...
10 | }
   | - mutable borrow ends here

error[E0506]: cannot assign to `next` because it is borrowed
 --> src/main.rs:8:9
  |
7 |         cur = next;
  |               ---- borrow of `next` occurs here
8 |         next = iter.next().unwrap();
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `next` occurs here

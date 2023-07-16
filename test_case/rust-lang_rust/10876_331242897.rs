
error[E0499]: cannot borrow `p.0` as mutable more than once at a time
  --> src/main.rs:10:25
   |
10 |             &mut Nat::S(ref mut n) => p = &mut *n
   |                         ^^^^^^^^^ mutable borrow starts here in previous iteration of loop
...
13 | }
   | - mutable borrow ends here

error[E0506]: cannot assign to `p` because it is borrowed
  --> src/main.rs:10:39
   |
10 |             &mut Nat::S(ref mut n) => p = &mut *n
   |                         ---------     ^^^^^^^^^^^ assignment to borrowed `p` occurs here
   |                         |
   |                         borrow of `p` occurs here


error[E0506]: cannot assign to `v[..]` because it is borrowed
  --> foo.rs:15:5
   |
9  |     let p = &v[0];
   |              ---- borrow of `v[..]` occurs here
...
15 |     v[0] += 1;
   |     ^^^^^^^^^ assignment to borrowed `v[..]` occurs here

error[E0506]: cannot assign to `v` because it is borrowed (Mir)
  --> foo.rs:16:2
   |
9  |     let p = &v[0];
   |             ----- borrow of `v` occurs here
...
16 | }
   |  ^ assignment to borrowed `v` occurs here

error: aborting due to 2 previous errors

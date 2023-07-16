
Mr-Darcy. rustc +nightly -Zborrowck-mir region-liveness-basic.rs
error[E0506]: cannot assign to `v[..]` because it is borrowed (Ast)
  --> region-liveness-basic.rs:32:5
   |
26 |     let p = &v[0];
   |              ---- borrow of `v[..]` occurs here
...
32 |     v[0] += 1;
   |     ^^^^^^^^^ assignment to borrowed `v[..]` occurs here

error: aborting due to previous error

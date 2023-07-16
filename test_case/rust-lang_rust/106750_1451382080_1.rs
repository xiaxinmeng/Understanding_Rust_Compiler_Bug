
error[E0505]: cannot move out of `a` because it is borrowed
  --> src/lib.rs:19:10
   |
14 |     let a: Vec<u8> = vec![0, 1, 2, 3];
   |         - binding `a` declared here
15 |     
16 |     let b = compute1(a.as_slice()); // ERROR
   |                      ------------ borrow of `a` occurs here
...
19 |     drop(a);
   |          ^ move out of `a` occurs here
20 |     dbg!(b.as_ref());
   |          ---------- borrow later used here

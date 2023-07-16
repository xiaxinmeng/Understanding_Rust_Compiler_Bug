`
error[E0505]: cannot move out of `v` because it is borrowed
  --> ices/97389.rs:33:14
   |
31 |     let el = &v[0];
   |               - borrow of `v` occurs here
32 |
33 |     for _ in v {
   |              ^ move out of `v` occurs here
34 |         // this triggers bug
35 |         println!("{}", ***el > 0);
   |                         ---- borrow later used here

error: aborting due to previous error


error[E0505]: cannot move out of `other` because it is borrowed
  --> src/main.rs:22:17
   |
17 |         *other = match (*other).get_self() {
   |                        -------- borrow of `**other` occurs here
...
22 |         let c = other;
   |                 ^^^^^ move out of `other` occurs here


error[E0505]: cannot move out of `fancy_num` because it is borrowed
  --> src/main.rs:11:17
   |
9  |       let fancy_ref = &fancy_num;
   |                       ---------- borrow of `fancy_num` occurs here
10 | 
11 |       let mut x = move || {
   |  _________________^
12 | |         fancy_num.num += 1;
13 | |     };
   | |_____^ move out of `fancy_num` occurs here
...
17 |       drop(fancy_ref);
   |            --------- borrow later used here

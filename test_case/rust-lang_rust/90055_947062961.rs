
error[E0505]: cannot move out of `r` because it is borrowed
 --> src/main.rs:5:14
  |
4 |     unsafe { std::ptr::replace::<&mut i32>(&mut r as *mut _, r); }
  |                                                              - borrow of `*r` occurs here
5 |     let v2 = r;
  |              ^
  |              |
  |              move out of `r` occurs here
  |              borrow later used here

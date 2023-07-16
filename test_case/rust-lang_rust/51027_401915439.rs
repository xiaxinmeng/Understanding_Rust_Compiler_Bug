
error: borrowed data cannot be stored outside of its closure
  --> issue-45983.rs:17:27
   |
16 |     let mut x = None;
   |         ----- `x` is declared outside of the closure body
17 |     give_any(|y| x = Some(y));:
   |               -  ^^^^^^^^^^^ can't store `y` into `x` because it's outside of its closure
   |               |
   |               this a borrow only valid for the duration of its closure body

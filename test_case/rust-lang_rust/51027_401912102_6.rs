
error: borrowed data cannot be stored outside of its closure
  --> issue-45983.rs:17:27
   |
17 |     give_any(|y| x = Some(y));:
   |               -  ^^^^^^^^^^^ stores `y` outside of the closure body
   |               |
   |               `y` is a reference that is only valid in the closure body

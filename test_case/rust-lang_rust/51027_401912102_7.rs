
error: borrowed data cannot be stored outside of its closure
  --> issue-45983.rs:17:27
   |
16 |     let mut x = None;
   |         ----- `x` is declared here, outside of the closure body
17 |     give_any(|y| x = Some(y));:
   |               -  ^^^^^^^^^^^ stores `y` into `x`
   |               |
   |               `y` is a reference that is only valid in the closure body

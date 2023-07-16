
error: unsatisfied lifetime constraints
  --> borrowck/issue-45983.rs:17:18
   |
16 |     let x = None;
   |         - region `'2` appears in the type of `x`
17 |     give_any(|y| x = Some(y));
   |               -  ^^^^^^^^^^^ free region requires that `'1` must outlive `'2`
   |               |
   |               region `'1` appears in this argument

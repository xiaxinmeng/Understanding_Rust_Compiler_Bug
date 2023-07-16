
error: unsatisfied lifetime constraints
 --> /home/nmatsakis/tmp/issue-51027.rs:7:18
  |
6 |     let x = None;
  |         - lifetime `'2` appears in the type of `x`
7 |     give_any(|y| x = Some(y));
  |               -  ^^^^^^^^^^^ free region requires that `'1` must outlive `'2`
  |               |
  |               lifetime `'1` appears in this argument

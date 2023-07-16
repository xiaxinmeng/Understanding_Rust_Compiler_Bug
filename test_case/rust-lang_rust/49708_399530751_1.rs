
error[E0512]: transmute called with types of different sizes
 --> src/lib.rs:6:14
  |
6 |     unsafe { transmute(a) }
  |              ^^^^^^^^^
  |
  = note: source type: [[T; 2]; 2] (size can vary because of T)
  = note: target type: [T; 4] (size can vary because of T)

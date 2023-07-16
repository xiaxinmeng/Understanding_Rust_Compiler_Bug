
error: unsatisfied lifetime constraints
 --> src/main.rs:6:47
  |
6 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |                                       ------- ^ returning this value requires that `'1` must outlive `'2`
  |                                       |     |
  |                                       |     return type of closure is &'2 &i32
  |                                       has type `&'1 (&i32, &i32)`

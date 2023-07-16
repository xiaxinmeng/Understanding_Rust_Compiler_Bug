
2 |     opt.map(|x| x.to_string()).unwrap_or_else(String::new)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ move occurs because `*opt` has type `Option<Box<i32>>`, which does not implement the `Copy` trait

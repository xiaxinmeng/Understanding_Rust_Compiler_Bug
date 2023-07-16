
error: implementation of `FnOnce` is not general enough
 --> src/main.rs:6:5
  |
6 |     do_op(path, "remove file", std::fs::remove_file); // redundant closure
  |     ^^^^^ implementation of `FnOnce` is not general enough
  |
  = note: `fn(&'2 Path) -> Result<(), std::io::Error> {remove_file::<&'2 Path>}` must implement `FnOnce<(&'1 Path,)>`, for any lifetime `'1`...
  = note: ...but it actually implements `FnOnce<(&'2 Path,)>`, for some specific lifetime `'2`


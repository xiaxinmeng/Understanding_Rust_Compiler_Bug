
error: invalid suffix `z` for number literal
 --> j.rs:1:6
  |
1 | #![a=5z]
  |      ^^ invalid suffix `z`
  |
  = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)

error: cannot find attribute `a` in this scope
 --> j.rs:1:4
  |
1 | #![a=5z]
  |    ^

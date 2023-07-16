
error[E0109]: type parameters are not allowed on this type
 --> test.rs:1:29
  |
1 | fn is_copy<T: ::std::marker<i32>::Copy>() {}
  |                             ^^^ type parameter not allowed

error: aborting due to previous error(s)

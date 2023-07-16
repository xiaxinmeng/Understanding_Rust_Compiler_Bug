
error: negative trait bounds are not supported
 --> file.rs:3:36
  |
3 | impl<T> Thing for Option<T> where T: !u64 {}
  |                                    ^^^^^^ help: remove the trait bound

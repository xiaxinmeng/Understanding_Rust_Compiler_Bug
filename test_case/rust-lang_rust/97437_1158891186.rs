plain
......................................................................iiiiii............ 1144/1211
...................................i...............................
failures:

---- src/os/fd/owned.rs - os::fd::owned::crate::sync::Arc<T> (line 361) stdout ----
error: the feature `io_safety` has been stable since 1.63.0 and no longer requires an attribute to enable
  |
  |
3 | #![feature(io_safety)]
  |
note: the lint level is defined here
 --> src/os/fd/owned.rs:359:9
  |

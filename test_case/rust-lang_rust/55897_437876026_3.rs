
use std::env;

include!(concat!(std::env!("OUT_DIR"), "/data.rs"));

error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(std::env!("OUT_DIR"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

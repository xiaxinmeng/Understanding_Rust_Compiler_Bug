
use std::env;

include!(concat!(env!("INVALID"), "/data.rs"));

error: environment variable `INVALID` not defined
 --> src/foo.rs:3:18
  |
3 | include!(concat!(env!("INVALID"), "/data.rs"));
  |                  ^^^^^^^^^^^^^^^
error: couldn't read "src/0/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(env!("INVALID"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

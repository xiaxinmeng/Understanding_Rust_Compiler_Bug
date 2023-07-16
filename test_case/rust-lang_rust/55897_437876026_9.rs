
use std::env;

include!(concat!(env!("INVALID"), "/data.rs"));

error: environment variable `INVALID` not defined
 --> src/foo.rs:3:18
  |
3 | include!(concat!(env!("INVALID"), "/data.rs"));
  |                  ^^^^^^^^^^^^^^^



use std::env;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

error[E0432]: unresolved import `prelude`
 --> src/main.rs:6:5
  |
6 | use prelude::*;
  |     ^^^^^^^ Did you mean `std::prelude`?


use std::env;

include!(concat!(std::env!("INVALID"), "/data.rs"));

error[E0432]: unresolved import `prelude`
 --> src/main.rs:6:5
  |
6 | use prelude::*;
  |     ^^^^^^^ Did you mean `std::prelude`?
error[E0433]: failed to resolve. Could not find `env` in `std`
 --> src/foo.rs:3:23
  |
3 | include!(concat!(std::env!("INVALID"), "/data.rs"));
  |                       ^^^ Could not find `env` in `std`

error: aborting due to 2 previous errors

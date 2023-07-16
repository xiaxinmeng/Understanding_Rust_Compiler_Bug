plain
   Doc-tests std

running 1227 tests
i................................................................................................... 100/1227
..................................F......F.......................................................... 200/1227
.................................................................................................... 400/1227
.....................ii............................................................................. 500/1227
...i....i................................................................i......................ii.. 600/1227
.................................................................................................... 700/1227
---
...................................................iiiiii......................................i.... 1200/1227
...........................
failures:

---- src/env.rs - env::set (line 361) stdout ----
error[E0658]: use of unstable library feature 'unsafe_env'
  |
  |
7 | unsafe { env::set(key, "VALUE"); } // Make sure you're single-threaded!
  |
  = help: add `#![feature(unsafe_env)]` to the crate attributes to enable

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---- src/env.rs - env::unset (line 433) stdout ----
error[E0658]: use of unstable library feature 'unsafe_env'
  |
  |
7 | unsafe { env::set(key, "VALUE"); } // Make sure you're single-threaded!
  |
  = help: add `#![feature(unsafe_env)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unsafe_env'
   |
   |
10 | unsafe { env::unset(key); } // Make sure you're single-threaded!
   |
   = help: add `#![feature(unsafe_env)]` to the crate attributes to enable

error: aborting due to 2 previous errors

plain
test src\thread\mod.rs - thread::spawn (line 606) ... ok

failures:

---- src\os\fd\raw.rs - os::fd::raw::crate::sync::Arc<T> (line 228) stdout ----
error[E0433]: failed to resolve: could not find `unix` in `os`
  |
6 | use std::os::unix::prelude::AsRawFd;
6 | use std::os::unix::prelude::AsRawFd;
  |              ^^^^ could not find `unix` in `os`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
---
test result: FAILED. 1074 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 36.15s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:45:38
make: *** [Makefile:70: ci-subset-1] Error 1

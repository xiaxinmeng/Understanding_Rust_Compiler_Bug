plain
...............................................................................................i.... 500/1191
i................................................................i.................................. 600/1191
.................................................................................................... 700/1191
.................................................................................................... 800/1191
.................F..FF.............................................................................. 900/1191
.................................................................................................... 1100/1191
....................iiiiii..................................i..............................
failures:


---- src/os/./unix/fs.rs - os::doc::unix::fs::chown (line 941) stdout ----
error[E0658]: use of unstable library feature 'unix_chown'
  |
  |
6 |     fs::chown("/sandbox", Some(0), Some(0))?;
  |
  |
  = help: add `#![feature(unix_chown)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/os/./unix/fs.rs - os::doc::unix::fs::fchown (line 960) stdout ----
error[E0252]: the name `fs` is defined multiple times
  |
3 | use std::os::unix::fs;
3 | use std::os::unix::fs;
  |     ----------------- previous import of the module `fs` here
4 | use std::os::unix::fs;
  |     ^^^^^^^^^^^^^^^^^ `fs` reimported here
  |
  = note: `fs` must be defined only once in the type namespace of this module
error: unused import: `std::os::unix::fs`
 --> src/os/./unix/fs.rs:962:5
  |
4 | use std::os::unix::fs;
---
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_imports)]` implied by `#[deny(warnings)]`

error[E0658]: use of unstable library feature 'unix_chown'
  |
  |
8 |     fs::fchown(f, Some(0), Some(0))?;
  |
  |
  = help: add `#![feature(unix_chown)]` to the crate attributes to enable
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0252, E0658.
For more information about an error, try `rustc --explain E0252`.
For more information about an error, try `rustc --explain E0252`.
Couldn't compile the test.
---- src/os/./unix/fs.rs - os::doc::unix::fs::lchown (line 982) stdout ----
error[E0658]: use of unstable library feature 'unix_chown'
  |
  |
6 |     fs::lchown("/symlink", Some(0), Some(0))?;
  |
  |
  = help: add `#![feature(unix_chown)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/os/./unix/fs.rs - os::doc::unix::fs::chown (line 941)
    src/os/./unix/fs.rs - os::doc::unix::fs::fchown (line 960)
    src/os/./unix/fs.rs - os::doc::unix::fs::lchown (line 982)
test result: FAILED. 1167 passed; 3 failed; 21 ignored; 0 measured; 0 filtered out; finished in 15.79s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:17:27

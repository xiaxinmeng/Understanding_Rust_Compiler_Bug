plain
---- src/path.rs - path::PathBuf::and_join (line 1413) stdout ----
error[E0658]: use of unstable library feature 'path_and_join'
 --> src/path.rs:1416:34
  |
6 | assert_eq!(PathBuf::from("/etc").and_join("passwd"), PathBuf::from("/etc/passwd"));
  |
  |
  = help: add `#![feature(path_and_join)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'path_and_join'
 --> src/path.rs:1419:34
  |
  |
9 | assert_eq!(PathBuf::from("/etc").and_join("passwd"), Path::new("/etc").join("passwd"));
  |
  |
  = help: add `#![feature(path_and_join)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'path_and_join'
  --> src/path.rs:1420:34
   |
   |
10 | assert_eq!(PathBuf::from("/etc").and_join("passwd"), {
   |
   |
   = help: add `#![feature(path_and_join)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:21:33

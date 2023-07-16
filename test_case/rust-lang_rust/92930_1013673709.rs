plain
---- src/ffi/os_str.rs - ffi::os_str::OsStr::from_str (line 639) stdout ----
error[E0658]: use of unstable library feature 'const_path': TBD
 --> src/ffi/os_str.rs:642:24
  |
6 | const OS_STR: &OsStr = OsStr::from_str("foo");
  |
  = help: add `#![feature(const_path)]` to the crate attributes to enable

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---- src/path.rs - path::Path::from_os_str (line 1961) stdout ----
error[E0658]: use of unstable library feature 'const_path': TBD
 --> src/path.rs:1965:21
  |
7 | const PATH: &Path = Path::from_os_str(OsStr::from_str("/foo/bar"));
  |
  = help: add `#![feature(const_path)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'const_path': TBD
error[E0658]: use of unstable library feature 'const_path': TBD
 --> src/path.rs:1965:39
  |
7 | const PATH: &Path = Path::from_os_str(OsStr::from_str("/foo/bar"));
  |
  = help: add `#![feature(const_path)]` to the crate attributes to enable

error: aborting due to 2 previous errors
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:21:29

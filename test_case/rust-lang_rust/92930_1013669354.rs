plain
---- src/ffi/os_str.rs - ffi::os_str::OsStr::from_str (line 639) stdout ----
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/ffi/os_str.rs:642:15
  |
6 | const OS_STR: OsStr = OsStr::from_str("foo");
  |
  |
  = help: within `OsStr`, the trait `Sized` is not implemented for `[u8]`
  = note: required because it appears within the type `OsStr`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
Couldn't compile the test.
---- src/path.rs - path::Path::from_os_str (line 1961) stdout ----
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/path.rs:1965:13
  |
7 | const PATH: Path = Path::from_os_str(OsStr::from_str("/foo/bar"));
  |
  = help: within `Path`, the trait `Sized` is not implemented for `[u8]`
  = note: required because it appears within the type `Path`

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:23:50

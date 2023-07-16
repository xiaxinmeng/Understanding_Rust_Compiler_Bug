plain
.................F...................................................................iiii........... 1100/1153
.....................................................
failures:

---- src/sys/unix/ext/fs.rs - sys::unix::ext::fs::chroot (line 897) stdout ----
error[E0658]: use of unstable library feature 'unix_chroot'
  |
  |
6 |     fs::chroot("/sandbox")?;
  |
  = note: see issue #84715 <https://github.com/rust-lang/rust/issues/84715> for more information
  = note: see issue #84715 <https://github.com/rust-lang/rust/issues/84715> for more information
  = help: add `#![feature(unix_chroot)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 0 --config /config/nopt-std-config.toml library/std
Build completed unsuccessfully in 0:01:44

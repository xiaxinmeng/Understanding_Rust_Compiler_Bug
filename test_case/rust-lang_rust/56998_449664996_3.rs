
error[E0460]: found possibly newer version of crate `std` which `getopts` depends on
  --> src/libtest/lib.rs:45:1
   |
45 | extern crate getopts;
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /d/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-5a6b2941d398f5d5.rlib
           crate `std`: /d/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-5a6b2941d398f5d5.dylib
           crate `getopts`: /d/rust/build/x86_64-apple-darwin/stage0-test/x86_64-apple-darwin/release/deps/libgetopts-8c1bca47bf6894d5.rlib

error: aborting due to previous error

For more information about this error, try `rustc --explain E0460`.
error: Could not compile `test`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/d/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "8" "--release" "--manifest-path" "/d/rust/src/libtest/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101
failed to run: /d/rust/build/bootstrap/debug/bootstrap test --stage 0 src/liballoc --test-args to_nonnull
Build completed unsuccessfully in 0:00:17

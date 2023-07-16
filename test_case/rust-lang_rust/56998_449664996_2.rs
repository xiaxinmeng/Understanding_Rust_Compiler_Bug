
error[E0460]: found possibly newer version of crate `alloc` which `std` depends on
   --> src/liballoc/lib.rs:128:1
    |
128 | extern crate std;
    | ^^^^^^^^^^^^^^^^^
    |
    = note: perhaps that crate needs to be recompiled?
    = note: the following crate versions were found:
            crate `alloc`: /d/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/liballoc-755c123e9f141b95.rlib
            crate `alloc`: /d/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/liballoc-755c123e9f141b95.rlib
            crate `std`: /d/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-5a6b2941d398f5d5.dylib
            crate `std`: /d/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libstd-5a6b2941d398f5d5.rlib

error: aborting due to previous error

For more information about this error, try `rustc --explain E0460`.
error: Could not compile `alloc`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/d/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-j" "8" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "/d/rust/src/libstd/Cargo.toml" "-p" "alloc" "--" "to_nonnull" "--quiet"
expected success, got: exit code: 101


failed to run: /d/rust/build/bootstrap/debug/bootstrap test --keep-stage 0 --stage 0 src/liballoc --test-args to_nonnull
Build completed unsuccessfully in 0:00:03

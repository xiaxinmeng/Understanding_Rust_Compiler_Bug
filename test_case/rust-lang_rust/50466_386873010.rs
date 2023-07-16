
$ ./x.py test --stage 0 --no-doc src/libcore --test-args time
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (file:///home/lukas/dev/rust/src/libcore)
   Compiling compiler_builtins v0.0.0 (file:///home/lukas/dev/rust/src/rustc/compiler_builtins_shim)
   Compiling libc v0.0.0 (file:///home/lukas/dev/rust/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///home/lukas/dev/rust/src/liballoc)
   Compiling std_unicode v0.0.0 (file:///home/lukas/dev/rust/src/libstd_unicode)
   Compiling unwind v0.0.0 (file:///home/lukas/dev/rust/src/libunwind)
   Compiling alloc_system v0.0.0 (file:///home/lukas/dev/rust/src/liballoc_system)
   Compiling panic_abort v0.0.0 (file:///home/lukas/dev/rust/src/libpanic_abort)
   Compiling alloc_jemalloc v0.0.0 (file:///home/lukas/dev/rust/src/liballoc_jemalloc)
   Compiling rustc_lsan v0.0.0 (file:///home/lukas/dev/rust/src/librustc_lsan)
   Compiling panic_unwind v0.0.0 (file:///home/lukas/dev/rust/src/libpanic_unwind)
   Compiling rustc_asan v0.0.0 (file:///home/lukas/dev/rust/src/librustc_asan)
   Compiling rustc_msan v0.0.0 (file:///home/lukas/dev/rust/src/librustc_msan)
   Compiling rustc_tsan v0.0.0 (file:///home/lukas/dev/rust/src/librustc_tsan)
   Compiling std v0.0.0 (file:///home/lukas/dev/rust/src/libstd)
    Finished release [optimized] target(s) in 30.15 secs
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (file:///home/lukas/dev/rust/src/libterm)
   Compiling getopts v0.2.17
   Compiling test v0.0.0 (file:///home/lukas/dev/rust/src/libtest)
    Finished release [optimized] target(s) in 6.58 secs
Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Testing core stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (file:///home/lukas/dev/rust/src/libcore)
error[E0460]: found possibly newer version of crate `std` which `rand` depends on
  --> libcore/../libcore/tests/lib.rs:53:1
   |
53 | extern crate rand;
   | ^^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: /home/lukas/dev/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-5f3a747b9ee6cbda.rlib
           crate `std`: /home/lukas/dev/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5f3a747b9ee6cbda.rlib
           crate `std`: /home/lukas/dev/rust/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5f3a747b9ee6cbda.so
           crate `std`: /home/lukas/dev/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-5f3a747b9ee6cbda.so
           crate `rand`: /home/lukas/dev/rust/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librand-89787ed220814c67.rlib

error: aborting due to previous error

For more information about this error, try `rustc --explain E0460`.
error: Could not compile `core`.
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/home/lukas/dev/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "8" "--release" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/home/lukas/dev/rust/src/libstd/Cargo.toml" "--lib" "--bins" "--examples" "--tests" "--benches" "-p" "core" "--" "time"
expected success, got: exit code: 101


failed to run: /home/lukas/dev/rust/build/bootstrap/debug/bootstrap test --stage 0 --no-doc src/libcore --test-args time
Build completed unsuccessfully in 0:00:44

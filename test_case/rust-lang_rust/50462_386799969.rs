plain
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:15]    Compiling libc v0.2.40
[01:10:16]    Compiling rand v0.4.2
[01:10:19]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:10:30] error: unused return value of `std::string::String::split_off` which must be used
[01:10:30]     |
[01:10:30]     |
[01:10:30] 247 |     split.split_off(orig.len() + 1);
[01:10:30]     |
[01:10:30]     = note: `-D unused-must-use` implied by `-D warnings`
[01:10:30] 
[01:10:30] 
[01:10:30] error: unused return value of `std::string::String::split_off` which must be used
[01:10:30]     |
[01:10:30]     |
[01:10:30] 254 |     orig.split_off(1);
[01:10:30] 
[01:10:30] error: aborting due to 2 previous errors
[01:10:30] 
[01:10:30] error: Could not compile `alloc`.
[01:10:30] error: Could not compile `alloc`.
[01:10:30] 
[01:10:30] To learn more, run the command again with --verbose.
[01:10:30] 
[01:10:30] 
[01:10:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:10:30] 
[01:10:30] 
[01:10:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:30] Build completed unsuccessfully in 0:26:40
[01:10:30] Build completed unsuccessfully in 0:26:40
[01:10:30] Makefile:58: recipe for target 'check' failed
[01:10:30] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2a7ba538
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
60840 ./src/llvm-emscripten/lib
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55380 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53660 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33ta18b3panbi
53656 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33ta18b3panbi/s-f0r1w561vd-1iks737-9sj2cmggjcjg
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47892 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
47052 ./src/test
46720 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release

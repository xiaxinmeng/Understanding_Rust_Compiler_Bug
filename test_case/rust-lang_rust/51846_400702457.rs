plain
[00:04:07]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:04:08]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:09]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:13]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:19] warning: function is never used: `hashmap_random_keys`
[00:04:19]   --> libstd/sys/unix/rand.rs:14:1
[00:04:19]    |
[00:04:19] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:04:19]    |
[00:04:19]    = note: #[warn(dead_code)] on by default
[00:04:19] 
[00:04:19] warning: function is never used: `getrandom`
[00:04:19] warning: function is never used: `getrandom`
[00:04:19]   --> libstd/sys/unix/rand.rs:36:5
[00:04:19]    |
[00:04:19] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:04:19] 
[00:04:19] warning: function is never used: `getrandom_fill_bytes`
[00:04:19]   --> libstd/sys/unix/rand.rs:45:5
[00:04:19]    |
[00:04:19]    |
[00:04:19] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:04:19] 
[00:04:19] 
[00:04:19] warning: function is never used: `is_getrandom_available`
[00:04:19]   --> libstd/sys/unix/rand.rs:67:5
[00:04:19]    |
[00:04:19] 67 |     fn is_getrandom_available() -> bool {
[00:04:19] 
[00:04:19] warning: function is never used: `fill_bytes`
[00:04:19]   --> libstd/sys/unix/rand.rs:93:5
[00:04:19]    |
[00:04:19]    |
[00:04:19] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:04:19] 
[00:04:28]     Finished release [optimized] target(s) in 51.77s
[00:04:28] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:28] travis_fold:end:stage0-std
---
[00:50:42] ....................................................................................................
[00:50:47] ....................................................................................................
[00:50:54] ....................................................................................................
[00:50:59] ....................................................................................................
[00:51:06] .................F.....i............................................................................
[00:51:17] ....................................................................................................
[00:51:24] ....................................................................................................
[00:51:31] ...........................................i........................................................
[00:51:33] ............................
[00:51:33] ............................
[00:51:33] failures:
[00:51:33] 
[00:51:33] ---- [ui] ui/missing-allocator.rs stdout ----
[00:51:33] diff of stderr:
[00:51:33] 
[00:51:33] 1 error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.
[00:51:33] 2 
[00:51:33] - error: aborting due to previous error
[00:51:33] + error: language item required, but not found: `hashmap_random_keys`
[00:51:33] + error: aborting due to 2 previous errors
[00:51:33] 4 
[00:51:33] 5 
[00:51:33] 
[00:51:33] 
[00:51:33] 
[00:51:33] The actual stderr differed from the expected stderr.
[00:51:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-allocator/missing-allocator.stderr
[00:51:33] To update references, rerun the tests and pass the `--bless` flag
[00:51:33] To only update this specific test, also pass `--test-args missing-allocator.rs`
[00:51:33] error: 1 errors occurred comparing output.
[00:51:33] status: exit code: 101
[00:51:33] status: exit code: 101
[00:51:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing-allocator.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-allocator/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-allocator/auxiliary" "-A" "unused"
[00:51:33] ------------------------------------------
[00:51:33] 
[00:51:33] ------------------------------------------
[00:51:33] stderr:
[00:51:33] stderr:
[00:51:33] ------------------------------------------
[00:51:33] {"message":"no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.","code":null,"level":"error","spans":[],"children":[],"rendered":"error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.\n\n"}
[00:51:33] {"message":"language item required, but not found: `hashmap_random_keys`","code":null,"level":"error","spans":[],"children":[],"rendered":"error: language item required, but not found: `hashmap_random_keys`\n\n"}
[00:51:33] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:33] ------------------------------------------
[00:51:33] 
[00:51:33] thread '[ui] ui/missing-allocator.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:51:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:51:33] 
[00:51:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:33] 
[00:51:33] 
[00:51:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:51:33] 
[00:51:33] 
[00:51:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:51:33] Build completed unsuccessfully in 0:02:24
[00:51:33] Build completed unsuccessfully in 0:02:24
[00:51:33] make: *** [check] Error 1
[00:51:33] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ba15444
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:18997d6c:start=1530111276899158399,finish=1530111276908902755,duration=9744356
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f1a4845
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01a5ab20
$ dmesg | grep -i kill

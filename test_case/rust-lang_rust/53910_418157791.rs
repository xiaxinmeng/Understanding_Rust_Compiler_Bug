plain
[00:04:51]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:04:52]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:53]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:04:54]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:59] warning: type `c_void` should have a camel case name such as `CVoid`
[00:04:59]   --> libcore/ffi.rs:28:1
[00:04:59]    |
[00:04:59] 28 | / pub enum c_void {
[00:04:59] 29 | |     #[unstable(feature = "c_void_variant", reason = "should not have to exist",
[00:04:59] 30 | |                issue = "0")]
[00:04:59] 31 | |     #[doc(hidden)] __variant1,
[00:04:59] ...  |
[00:04:59] 34 | |     #[doc(hidden)] __variant2,
[00:04:59]    | |_^
[00:04:59]    |
[00:04:59] note: lint level defined here
[00:04:59]   --> libcore/ffi.rs:3:9
[00:04:59]   --> libcore/ffi.rs:3:9
[00:04:59]    |
[00:04:59] 3  | #![warn(non_camel_case_types)]
[00:04:59] 
[00:04:59] 
[00:04:59] warning: variant `__variant1` should have a camel case name such as `Variant1`
[00:04:59]   --> libcore/ffi.rs:31:20
[00:04:59]    |
[00:04:59] 31 |     #[doc(hidden)] __variant1,
[00:04:59] 
[00:04:59] 
[00:04:59] warning: variant `__variant2` should have a camel case name such as `Variant2`
[00:04:59]   --> libcore/ffi.rs:34:20
[00:04:59]    |
[00:04:59] 34 |     #[doc(hidden)] __variant2,
[00:04:59] 
[00:05:07]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:05:07]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:05:08]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
---
[00:21:14]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:15]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:21:15]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:16]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:41] error: type `c_void` should have a camel case name such as `CVoid`
[00:21:41]   --> libcore/ffi.rs:28:1
[00:21:41]    |
[00:21:41] 28 | / pub enum c_void {
[00:21:41] 29 | |     #[unstable(feature = "c_void_variant", reason = "should not have to exist",
[00:21:41] 30 | |                issue = "0")]
[00:21:41] 31 | |     #[doc(hidden)] __variant1,
[00:21:41] ...  |
[00:21:41] 34 | |     #[doc(hidden)] __variant2,
[00:21:41]    | |_^
[00:21:41]    |
[00:21:41]    = note: `-D non-camel-case-types` implied by `-D warnings`
[00:21:41] 
[00:21:41] 
[00:21:41] error: variant `__variant1` should have a camel case name such as `Variant1`
[00:21:41]   --> libcore/ffi.rs:31:20
[00:21:41]    |
[00:21:41] 31 |     #[doc(hidden)] __variant1,
[00:21:41] 
[00:21:41] 
[00:21:41] error: variant `__variant2` should have a camel case name such as `Variant2`
[00:21:41]   --> libcore/ffi.rs:34:20
[00:21:41]    |
[00:21:41] 34 |     #[doc(hidden)] __variant2,
[00:21:41] 
[00:21:41] error: aborting due to 3 previous errors
[00:21:41] 
[00:21:41] error: Could not compile `core`.
---
[00:21:41] travis_time:end:stage1-std:start=1535992315236917460,finish=1535992353549151273,duration=38312233813

[00:21:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:41] expected success, got: exit code: 101
[00:21:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:21:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:41] Build completed unsuccessfully in 0:17:03
[00:21:41] Makefile:28: recipe for target 'all' failed
[00:21:41] Makefile:28: recipe for target 'all' failed
[00:21:41] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1591c8c6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2ba46855:start=1535992354384614073,finish=1535992354394893789,duration=10279716
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ffb88c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15554e17
travis_time:start:15554e17
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0137bc53
$ dmesg | grep -i kill

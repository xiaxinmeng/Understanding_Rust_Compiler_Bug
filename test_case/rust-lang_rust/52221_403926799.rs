plain
[00:04:47]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:04:47]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:48]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:51]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:53] error: trait objects without an explicit `dyn` are deprecated
[00:04:53]   --> libstd/sys/unix/process/process_common.rs:55:23
[00:04:53]    |
[00:04:53] 55 |     closures: Vec<Box<FnMut() -> io::Result<()> + Send + Sync>>,
[00:04:53]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
[00:04:53] note: lint level defined here
[00:04:53]   --> libstd/lib.rs:224:9
[00:04:53]    |
[00:04:53]    |
[00:04:53] 224| #![deny(bare_trait_objects)]
[00:04:53] 
[00:04:53] 
[00:04:53] error: trait objects without an explicit `dyn` are deprecated
[00:04:53]    --> libstd/sys/unix/process/process_common.rs:158:52
[00:04:53]     |
[00:04:53] 158 |     pub fn get_closures(&mut self) -> &mut Vec<Box<FnMut() -> io::Result<()> + Send + Sync>> {
[00:04:53]     |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
[00:04:53] 
[00:04:53] error: trait objects without an explicit `dyn` are deprecated
[00:04:53]    --> libstd/sys/unix/process/process_common.rs:163:31
[00:04:53]     |
[00:04:53] 163 |                        f: Box<FnMut() -> io::Result<()> + Send + Sync>) {
[00:04:53]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
[00:04:53] 
[00:04:53] error: trait objects without an explicit `dyn` are deprecated
[00:04:53]   --> libstd/sys/unix/thread.rs:52:48
[00:04:53]    |
[00:04:53] 52 |     pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>)
[00:04:53]    |                                                ^^^^^^^^^^^^ help: use `dyn`: `dyn FnBox() + 'a`
[00:04:55] error: aborting due to 4 previous errors
[00:04:55] 
[00:04:55] error: Could not compile `std`.
[00:04:55] 
[00:04:55] 
[00:04:55] Caused by:
[00:04:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=acba8e5dff5ff58f -C extra-filename=-acba8e5dff5ff58f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-07d51360c24e2889.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-a8bbca5d1366afa9.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-37588f22ef119a7c.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-8c516c50de8b60ca.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-c8f7a210a5d40dd6.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-b89e8d3f043cfac8.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-e900ea439ca38db1.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-12c97d41730d2562.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-d70c8d57a4734636.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-338adff8fe86271b.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-de5e79e57985ddeb.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-2f619371d304061f.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-3bc1496f398a04ef.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-46b511c9b9d00ae8.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace -l static=backtrace -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-8c31df4670881c7a/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:04:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:55] expected success, got: exit code: 101
[00:04:55] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:04:55] travis_fold:end:stage0-std

[00:04:55] travis_time:end:stage0-std:start=1531248265189865180,finish=1531248300622290474,duration=35432425294


[00:04:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:55] Build completed unsuccessfully in 0:00:36
[00:04:55] make: *** [all] Error 1
[00:04:55] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1dd0fcd6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:05139c96:start=1531248301209456026,finish=1531248301217800272,duration=8344246
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d01a938
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1227770a
$ dmesg | grep -i kill

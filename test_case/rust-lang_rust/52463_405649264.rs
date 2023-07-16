plain
[00:19:07]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:19:08]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:19:08]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:19:13]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
eckout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-701e0fd8b43e7652.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-3243da5dca2f7f19.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-724f8224bf36828f.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-594134273e28a017.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-299df71ced1880ef.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-91594473ba138443.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-0373f17006917dc2.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-2e1519c7ca1bc19a.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-fbdf5a78257f065a.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-38f43a998a7cf346/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (signal: 11, SIGSEGV: invalid memory reference)
[00:19:25] expected success, got: exit code: 101
[00:19:25] expected success, got: exit code: 101
[00:19:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:19:25] travis_fold:end:stage1-std

[00:19:25] travis_time:end:stage1-std:start=1531845800193705501,finish=1531845875056250811,duration=74862545310


[00:19:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:25] Build completed unsuccessfully in 0:15:44
[00:19:25] Makefile:28: recipe for target 'all' failed
[00:19:25] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
ib/rustlib/x86_64-unknown-linux-gnu/lib
177640 ./obj/build/bootstrap/debug/deps
---
158004 ./.git/modules/src
149124 ./src/llvm-emscripten/test
145036 ./obj/build/bootstrap/debug/incremental
130516 ./obj/build/bootstrap/debug/incremental/bootstrap-3kaq1kqcanyi4
130512 ./obj/build/bootstrap/debug/incremental/bootstrap-3kaq1kqcanyi4/s-f2zrsunkwl-15jmlbv-xx3ruytk6cjw
103368 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
97536 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
87596 ./obj/build/x86_64-unknown-linux-gnu/stage1
87572 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
---
travis_time:end:07f3ea91:start=1531845875704893176,finish=1531845875714579765,duration=9686589
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bdcc35b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.9233.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc

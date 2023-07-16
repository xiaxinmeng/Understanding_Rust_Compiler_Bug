plain
[00:19:11]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:19:11]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:12]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:19:12]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:19:31] error: internal compiler error: librustc_mir/borrow_check/places_conflict.rs:193: Tracking borrow behind shared reference.
[00:19:31] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:599:9
[00:19:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
in 0:15:49
[00:19:31] Makefile:28: recipe for target 'all' failed
[00:19:31] Makefile:28: recipe for target 'all' failed
[00:19:31] make: *** [all] Error 1
1542188 ./obj
1542148 ./obj/build
1189700 ./.git
1070268 ./src
---
151412 ./src/tools/clang
151244 ./obj/build/bootstrap/debug/incremental
149116 ./src/llvm-emscripten/test
135792 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
135788 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f5t2m8spq2-hsqgkd-anymfpm2juvh
107664 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
95760 ./obj/build/x86_64-unknown-linux-gnu/stage1
95740 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
---
travis_time:end:0c12cb7c:start=1539784203438500619,finish=1539784203444266298,duration=5765679
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01eee241
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c7c288e
travis_time:start:1c7c288e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:094f44b8
$ dmesg | grep -i kill

plain
[00:53:21] ....................................................................................................
[00:53:24] ....................................................................................................
[00:53:27] ......i.............................................................................................
[00:53:30] ....................................................................................................
[00:53:32] .......................................................iiiiiiiii....................................
[00:53:38] ....................................................................................................
[00:53:42] ....................................................................................................
[00:53:45] ....................................i...............................................................
[00:53:47] ......................................................................................i.i..ii.......
---
[00:58:38] ....................................................................................................
[00:58:52] ....................................................................................................
[00:59:07] ..........................................................ii........................................
[00:59:24] ....................i....i.....................................................i....................
[00:59:52] ...........................F........................................................................
[01:00:11] ....................................................................................................
[01:00:22] ....................................................................................................
[01:00:31] ..........................................................................
[01:00:31] failures:
[01:00:31] failures:
[01:00:31] 
[01:00:31] ---- [run-pass] run-pass/super-fast-paren-parsing.rs stdout ----
[01:00:31] 
[01:00:31] error: compilation failed!
[01:00:31] status: exit code: 101
[01:00:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/super-fast-paren-parsing.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/super-fast-paren-parsing/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/super-fast-paren-parsing/auxiliary"
[01:00:31] ------------------------------------------
[01:00:31] 
[01:00:31] ------------------------------------------
[01:00:31] stderr:
[01:00:31] stderr:
[01:00:31] ------------------------------------------
[01:00:31] thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:186:9
[01:00:31] 
[01:00:31] error: internal compiler error: unexpected panic
[01:00:31] 
[01:00:31] note: the compiler unexpectedly panicked. this is a bug.
[01:00:31] note: the compiler unexpectedly panicked. this is a bug.
[01:00:31] 
[01:00:31] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:00:31] 
[01:00:31] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[01:00:31] 
[01:00:31] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[01:00:31] 
[01:00:31] ------------------------------------------
[01:00:31] 
[01:00:31] thread '[run-pass] run-pass/super-fast-paren-parsing.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
---
[01:00:31] 
[01:00:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:00:31] 
[01:00:31] 
[01:00:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:31] 
[01:00:31] 
[01:00:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:31] Build completed unsuccessfully in 0:11:21
[01:00:31] Build completed unsuccessfully in 0:11:21
[01:00:31] make: *** [check] Error 1
[01:00:31] Makefile:58: recipe for target 'check' failed
35328 ./.git/modules/src/libcompiler_builtins/modules
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
34832 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
34832 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
---
travis_time:end:22ac2df4:start=1534772673165503765,finish=1534772673173001974,duration=7498209
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b75feaa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11de5f50
travis_time:start:11de5f50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01957170
$ dmesg | grep -i kill

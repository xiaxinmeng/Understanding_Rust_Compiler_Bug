plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:09] 
[00:47:09] running 1567 tests
[00:47:12] ............F.....................................................................................i.
[00:47:19] ....................................................................................................
[00:47:21] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:26] ....................................................................................................
[00:47:26] ....................................................................................................
[00:47:29] ....................................................................................................
[00:47:32] ....................................................................................................
[00:47:36] ....................................................................................................
[00:47:39] ....................................................................................................
[00:47:43] ........................................i...........................................................
[00:47:46] ..............................i.....................................................................
[00:47:50] ....................................................................................................
[00:47:53] ...................................F................................................................
[00:48:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:48:00] ...................................................................
[00:48:00] failures:
[00:48:00] 
[00:48:00] 
[00:48:00] ---- [ui] ui/arbitrary-self-types-not-object-safe.rs stdout ----
[00:48:00] 
[00:48:00] error: ui test compiled successfully!
[00:48:00] status: exit code: 0
[00:48:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/arbitrary-self-types-not-object-safe.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-self-types-not-object-safe/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-seht_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: librustc/ty/subst.rs:425: Region parameter out of range when substituting in region 'a (root type=Some(&'a Self)) (index=1)\n\n"}
[00:48:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:00] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:00] 
[00:48:00] note: the compiler unexpectedly panicked. this is a bug.
[00:48:00] 
[00:48:00] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:00] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:48:00] 
[00:48:00] 
[00:48:00] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:48:00] 
[00:48:00] ------------------------------------------
[00:48:00] 
[00:48:00] thread '[ui] ui/span/dropck-object-cycle.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
---
travis_time:end:12926cd0:start=1531696441269220248,finish=1531696441276135294,duration=6915046
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c5a4ec6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00375784
travis_time:start:00375784
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:093fa174
$ dmesg | grep -i kill

plain
travis_time:end:05fa25d4:start=1548711101806335765,finish=1548711105375543834,duration=3569208069
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:37] .................................................................................................... 1200/2946
[01:03:47] .................................................................................................... 1300/2946
[01:03:58] .................................................................................................... 1400/2946
[01:04:10] .................................................................................................... 1500/2946
[01:04:19] ......................................................................i.....................F....... 1600/2946
[01:04:44] .................................................................................................... 1800/2946
[01:04:54] ..................................................................................................i. 1900/2946
[01:05:08] .....................................................................i.............................. 2000/2946
[01:05:32] .................................................................................................... 2100/2946
---
[01:07:39] failures:
[01:07:39] 
[01:07:39] ---- [run-pass] run-pass/issues/issue-50689.rs stdout ----
[01:07:39] 
[01:07:39] error: test compilation failed although it shouldn't!
[01:07:39] status: exit code: 101
[01:07:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-50689.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-50689/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-50689/auxiliary"
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] stderr:
[01:07:39] stderr:
[01:07:39] ------------------------------------------
[01:07:39] {"message":"src/librustc_mir/build/mod.rs:71: can't build MIR for DefId(0/0:3 ~ issue_50689[317d]::Foo[0])","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-50689.rs","byte_start":40,"byte_end":84,"line_start":3,"line_end":5,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"enum Foo {","highlight_start":1,"highlight_end":11},{"text":"    Bar = (|x: i32| { }, 42).1,","highlight_start":1,"highlight_end":32},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/build/mod.rs:71: can't build MIR for DefId(0/0:3 ~ issue_50689[317d]::Foo[0])\n  --> /checkout/src/test/run-pass/issues/issue-50689.rs:3:1\n   |\nLL | / enum Foo {\nLL | |     Bar = (|x: i32| { }, 42).1,\nLL | | }\n   | |_^\n\n"}
[01:07:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:39] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:07:39] 
[01:07:39] note: the compiler unexpectedly panicked. this is a bug.
[01:07:39] note: the compiler unexpectedly panicked. this is a bug.
[01:07:39] 
[01:07:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:07:39] 
[01:07:39] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:07:39] 
[01:07:39] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:07:39] 
[01:07:39] ------------------------------------------
[01:07:39] 
[01:07:39] thread '[run-pass] run-pass/issues/issue-50689.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3288:9
---
[01:07:39] 
[01:07:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:07:39] 
[01:07:39] 
[01:07:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:39] 
[01:07:39] 
[01:07:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:39] Build completed unsuccessfully in 0:10:06
[01:07:39] Build completed unsuccessfully in 0:10:06
[01:07:39] Makefile:48: recipe for target 'check' failed
[01:07:39] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04d8d054
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 28 22:39:35 UTC 2019
---
travis_time:end:023a2273:start=1548715176468227707,finish=1548715176472529059,duration=4301352
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fe4f289
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:238f13a4
travis_time:start:238f13a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06779d54
$ dmesg | grep -i kill

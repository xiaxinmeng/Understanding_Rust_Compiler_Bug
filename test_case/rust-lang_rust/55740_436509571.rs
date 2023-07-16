plain
travis_time:end:33d9e280:start=1541565257978533504,finish=1541565259031431238,duration=1052897734
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:28] .................................................................................................... 100/4998
[00:47:31] .................................................................................................... 200/4998
[00:47:34] ........................................................................ii...................ii..... 300/4998
[00:47:36] ...........................................................................................iii...... 400/4998
[00:47:39] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4998
[00:47:46] .................................................................................................... 700/4998
[00:47:52] .....................................................................i...........i.................. 800/4998
[00:47:55] ........................................................................................iiiii....... 900/4998
[00:47:58] ...........ii.iiii.................................................................................. 1000/4998
---
[00:48:32] .................................................................................................... 2200/4998
[00:48:36] .................................................................................................... 2300/4998
[00:48:39] .................................................................................................... 2400/4998
[00:48:43] .................................................................................................... 2500/4998
[00:48:46] ..............................................................................iiiiiiiii............. 2600/4998
[00:48:52] .............................ii..................................................................... 2800/4998
[00:48:55] .................................................................................................... 2900/4998
[00:48:58] .................................................................................................... 3000/4998
[00:49:01] ........................i........................................................................... 3100/4998
---
[00:53:15] ........................................................................i........................... 1900/2881
[00:53:26] ..........................................i......................................................... 2000/2881
[00:53:48] .................................................................................................... 2100/2881
[00:53:55] .................................................................................................... 2200/2881
[00:54:11] ....ii............F........................................................i....i................... 2300/2881
[00:54:36] .................................................................................................... 2500/2881
[00:55:04] .................................................................................................... 2600/2881
[00:55:13] .................................................................................................... 2700/2881
[00:55:21] .................................................................................................... 2800/2881
[00:55:21] .................................................................................................... 2800/2881
[00:55:32] .................................................................................
[00:55:32] failures:
[00:55:32] 
[00:55:32] ---- [run-pass] run-pass/rfcs/rfc-2302-self-struct-ctor.rs stdout ----
[00:55:32] 
[00:55:32] error: test compilation failed although it shouldn't!
[00:55:32] status: exit code: 101
[00:55:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rfcs/rfc-2302-self-struct-ctor.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2302-self-struct-ctor/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfcs/rfc-2302-self-struct-ctor/auxiliary"
[00:55:32] ------------------------------------------
[00:55:32] 
[00:55:32] ------------------------------------------
[00:55:32] stderr:
[00:55:32] stderr:
[00:55:32] ------------------------------------------
[00:55:32] {"message":"librustc_typeck/check/_match.rs:820: unexpected pattern definition: SelfCtor(DefId(0/0:5 ~ rfc_2302_self_struct_ctor[317d]::{{impl}}[0]))","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_typeck/check/_match.rs:820: unexpected pattern definition: SelfCtor(DefId(0/0:5 ~ rfc_2302_self_struct_ctor[317d]::{{impl}}[0]))\n\n"}
[00:55:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:32] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:55:32] note: the compiler unexpectedly panicked. this is a bug.
[00:55:32] 
[00:55:32] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:55:32] 
[00:55:32] 
[00:55:32] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:55:32] 
[00:55:32] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:55:32] 
[00:55:32] ------------------------------------------
[00:55:32] 
[00:55:32] thread '[run-pass] run-pass/rfcs/rfc-2302-self-struct-ctor.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
---
[00:55:32] 
[00:55:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:55:32] 
[00:55:32] 
[00:55:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:32] 
[00:55:32] 
[00:55:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:32] Build completed unsuccessfully in 0:11:33
[00:55:32] Build completed unsuccessfully in 0:11:33
[00:55:32] make: *** [check] Error 1
[00:55:32] Makefile:58: recipe for target 'check' failed
2472692 ./obj
2459944 ./obj/build
1824164 ./obj/build/x86_64-unknown-linux-gnu
1069604 ./src
---
142868 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
137084 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
137080 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6ftdb5ouf-1tnqsqn-22tmsi8iacpi9
130756 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130752 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123680 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
111072 ./src/llvm/test/CodeGen
---
55856 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
55852 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
55848 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
55844 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
55840 ./obj/build/x86_64-unkno\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:019379a8
travis_time:start:019379a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08847b07
$ dmesg | grep -i kill

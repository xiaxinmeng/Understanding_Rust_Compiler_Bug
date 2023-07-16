plain
travis_time:end:021315f3:start=1557312261456580734,finish=1557312348647757955,duration=87191177221
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:48] 
[01:22:48] running 139 tests
[01:22:51] i..iii.....iii..iiii.....i..............F.....i..i.................i.....i........F.ii.i..i..i.ii... 100/139
[01:22:53] failures:
[01:22:53] 
[01:22:53] ---- [codegen] codegen/inline-always-works-always.rs#NO-OPT stdout ----
[01:22:53] 
[01:22:53] 
[01:22:53] error in revision `NO-OPT`: verification with 'FileCheck' failed
[01:22:53] status: exit code: 1
[01:22:53] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-always-works-always.NO-OPT/inline-always-works-always.ll" "/checkout/src/test/codegen/inline-always-works-always.rs" "--check-prefixes" "CHECK,NO-OPT"
[01:22:53] ------------------------------------------
[01:22:53] 
[01:22:53] ------------------------------------------
[01:22:53] stderr:
[01:22:53] stderr:
[01:22:53] ------------------------------------------
[01:22:53] /checkout/src/test/codegen/inline-always-works-always.rs:17:12: error: expected string not found in input
[01:22:53] // NO-OPT: ret i32 8
[01:22:53]            ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-always-works-always.NO-OPT/inline-always-works-always.ll:14:19: note: scanning from here
[01:22:53] define i32 @caller() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:22:53]                   ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inline-always-works-always.NO-OPT/inline-always-works-always.ll:59:2: note: possible intended match here
[01:22:53]  ret i32 %12
[01:22:53] 
[01:22:53] ------------------------------------------
[01:22:53] 
[01:22:53] 
[01:22:53] 
[01:22:53] ---- [codegen] codegen/optimize-attr-1.rs#NO-OPT stdout ----
[01:22:53] 
[01:22:53] error in revision `NO-OPT`: verification with 'FileCheck' failed
[01:22:53] status: exit code: 1
[01:22:53] command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll" "/checkout/src/test/codegen/optimize-attr-1.rs" "--check-prefixes" "CHECK,NO-OPT"
[01:22:53] ------------------------------------------
[01:22:53] 
[01:22:53] ------------------------------------------
[01:22:53] stderr:
[01:22:53] stderr:
[01:22:53] ------------------------------------------
[01:22:53] /checkout/src/test/codegen/optimize-attr-1.rs:11:12: error: expected string not found in input
[01:22:53] // NO-OPT: ret i32 %1
[01:22:53]            ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:13:39: note: scanning from here
[01:22:53] define i32 @nothing() unnamed_addr #0 {
[01:22:53]                                       ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:29:2: note: possible intended match here
[01:22:53]  ret i32 %7
[01:22:53] /checkout/src/test/codegen/optimize-attr-1.rs:21:12: error: expected string not found in input
[01:22:53] // NO-OPT: ret i32 %1
[01:22:53]            ^
[01:22:53]            ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:38:36: note: scanning from here
[01:22:53] define i32 @size() unnamed_addr #1 {
[01:22:53]                                    ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:54:2: note: possible intended match here
[01:22:53]  ret i32 %7
[01:22:53] /checkout/src/test/codegen/optimize-attr-1.rs:34:12: error: expected string not found in input
[01:22:53] // NO-OPT: ret i32 %1
[01:22:53]            ^
[01:22:53]            ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:63:37: note: scanning from here
[01:22:53] define i32 @speed() unnamed_addr #0 {
[01:22:53]                                     ^
[01:22:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:79:2: note: possible intended match here
[01:22:53]  ret i32 %7
[01:22:53] 
[01:22:53] ------------------------------------------
[01:22:53] 
[01:22:53] 
---
[01:22:53] 
[01:22:53] 
[01:22:53] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:22:53] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:22:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:53] 
[01:22:53] 
[01:22:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:53] Build completed unsuccessfully in 0:12:08
[01:22:53] Build completed unsuccessfully in 0:12:08
[01:22:53] Makefile:48: recipe for target 'check' failed
[01:22:53] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:005493db
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  8 12:08:51 UTC 2019
---
travis_time:end:2acd5fde:start=1557317332981799136,finish=1557317332986536334,duration=4737198
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e302d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08d4cd2d
travis_time:start:08d4cd2d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ad2b99c
$ dmesg | grep -i kill

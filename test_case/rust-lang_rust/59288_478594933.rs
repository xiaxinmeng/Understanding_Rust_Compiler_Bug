plain
travis_time:end:0d78435a:start=1554123221043397456,finish=1554123223492995302,duration=2449597846
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:50] .................................................................................................... 1400/5516
[01:12:54] .................................................................................................... 1500/5516
[01:12:57] .................................................................................................... 1600/5516
[01:13:00] ..................................................i................................................. 1700/5516
[01:13:04] ................................................................F......F............................ 1800/5516
[01:13:12] .................................................................................................... 2000/5516
[01:13:16] ......................................................................................i............. 2100/5516
[01:13:21] .................................................................................................... 2200/5516
[01:13:25] .................................................................................................... 2300/5516
[01:13:25] .................................................................................................... 2300/5516
[01:13:29] .................................................................................................... 2400/5516
[01:13:34] .................................................................................................... 2500/5516
[01:13:38] .................................................................................................... 2600/5516
[01:13:42] .................................................................................................... 2700/5516
[01:13:47] .................................................................................................... 2800/5516
[01:13:52] ....................................................................................F............... 2900/5516
[01:14:00] .................................................................................................... 3100/5516
[01:14:04] .................................................................................................... 3200/5516
[01:14:09] .................................................................................................... 3300/5516
[01:14:13] ..................i................................................................................. 3400/5516
---
[01:15:45] error: /checkout/src/test/ui/if/if-let-arm-types.rs:6: unexpected error: '6:9: 6:10: if and else have incompatible types [E0308]'
[01:15:45] 
[01:15:45] error: /checkout/src/test/ui/if/if-let-arm-types.rs:4: unexpected note: 'expected because of this'
[01:15:45] 
[01:15:45] error: /checkout/src/test/ui/if/if-let-arm-types.rs:2: unexpected note: 'if and else have incompatible types'
[01:15:45] error: /checkout/src/test/ui/if/if-let-arm-types.rs:2: expected note not found: if let` arms have incompatible types
[01:15:45] 
[01:15:45] error: /checkout/src/test/ui/if/if-let-arm-types.rs:6: expected error not found: `if let` arms have incompatible types
[01:15:45] 
[01:15:45] 
[01:15:45] error: 3 unexpected errors found, 2 expected errors not found
[01:15:45] status: exit code: 1
[01:15:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/if-let-arm-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-let-arm-types/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-let-arm-types/auxiliary" "-A" "unused"
[01:15:45]     Error {
[01:15:45]         line_num: 6,
[01:15:45]         kind: Some(
[01:15:45]             Error
---
[01:15:45]         line_num: 2,
[01:15:45]         kind: Some(
[01:15:45]             Note
[01:15:45]         ),
[01:15:45]         msg: "if and else have incompatible types"
[01:15:45] ]
[01:15:45] 
[01:15:45] not found errors (from test file): [
[01:15:45]     Error {
---
[01:15:45] error: /checkout/src/test/ui/if/if-without-else-as-fn-expr.rs:17: unexpected error: '17:5: 19:6: mismatched types [E0308]'
[01:15:45] 
[01:15:45] error: 3 unexpected errors found, 0 expected errors not found
[01:15:45] status: exit code: 1
[01:15:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/if/if-without-else-as-fn-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-as-fn-expr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-without-else-as-fn-expr/auxiliary" "-A" "unused"
[01:15:45]     Error {
[01:15:45]         line_num: 2,
[01:15:45]         kind: Some(
[01:15:45]             Error
---
[01:15:45] error: /checkout/src/test/ui/issues/issue-50577.rs:3: unexpected error: '3:16: 3:32: mismatched types [E0308]'
[01:15:45] 
[01:15:45] error: 1 unexpected errors found, 0 expected errors not found
[01:15:45] status: exit code: 1
[01:15:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50577.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50577/auxiliary" "-A" "unused"
[01:15:45]     Error {
[01:15:45]         line_num: 3,
[01:15:45]         kind: Some(
[01:15:45]             Error
---
[01:15:45] 
[01:15:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:15:45] 
[01:15:45] 
[01:15:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:45] 
[01:15:45] 
[01:15:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:45] Build completed unsuccessfully in 0:04:54
[01:15:45] Build completed unsuccessfully in 0:04:54
[01:15:45] Makefile:48: recipe for target 'check' failed
[01:15:45] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d37d0c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  1 14:09:39 UTC 2019
---
travis_time:end:084bf6e8:start=1554127781584540332,finish=1554127781590336704,duration=5796372
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08782668
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:194e9056
travis_time:start:194e9056
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02609569
$ dmesg | grep -i kill

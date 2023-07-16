plain
travis_time:end:19fbba29:start=1553616750327638211,finish=1553616752783961656,duration=2456323445
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:29] 
[01:13:29] running 5514 tests
[01:13:34] .......................................................................................F............ 100/5514
[01:13:41] .................................................................................................... 300/5514
[01:13:44] .................................................................................................... 400/5514
[01:13:48] .................................................................................................... 500/5514
[01:13:51] ..................................................................i................................. 600/5514
---
[01:17:05] failures:
[01:17:05] 
[01:17:05] ---- [ui] ui/associated-type-bounds/nested-lifetime-bounds.rs stdout ----
[01:17:05] 
[01:17:05] error: /checkout/src/test/ui/associated-type-bounds/nested-lifetime-bounds.rs:11: expected error not found: nested quantification of lifetimes [E0316]
[01:17:05] error: 0 unexpected errors found, 1 expected errors not found
[01:17:05] status: exit code: 1
[01:17:05] status: exit code: 1
[01:17:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/nested-lifetime-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/nested-lifetime-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/nested-lifetime-bounds/auxiliary" "-A" "unused"
[01:17:05]     Error {
[01:17:05]         line_num: 11,
[01:17:05]         kind: Some(
[01:17:05]             Error
[01:17:05]             Error
[01:17:05]         ),
[01:17:05]         msg: "nested quantification of lifetimes [E0316]"
[01:17:05] ]
[01:17:05] 
[01:17:05] 
[01:17:05] thread '[ui] ui/associated-type-bounds/nested-lifetime-bounds.rs' panicked at 'explicit panic', src/tools/compileonents" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:17:05] 
[01:17:05] 
[01:17:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:05] Build completed unsuccessfully in 0:04:40
[01:17:05] Build completed unsuccessfully in 0:04:40
[01:17:05] Makefile:48: recipe for target 'check' failed
[01:17:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a12a39a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 17:29:49 UTC 2019
---
travis_time:end:03e3a6be:start=1553621390585436004,finish=1553621390590972913,duration=5536909
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07ce5ea0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:007f05f5
travis_time:start:007f05f5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:315dbab0
$ dmesg | grep -i kill

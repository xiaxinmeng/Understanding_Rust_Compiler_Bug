plain
travis_time:end:14dfb382:start=1555844588644208428,finish=1555844590926008738,duration=2281800310
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:18] 
[01:14:18] running 9 tests
[01:14:18] iiiiiiiii
[01:14:18] 
[01:14:18]  finished in 0.152
[01:14:18] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:34] 
[01:14:34] running 121 tests
[01:15:00] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:04] i.i......iii.i.....ii
[01:15:04] 
[01:15:04]  finished in 30.181
[01:15:04] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:59] 
[01:18:59] running 304 tests
[01:20:10] .....F......................i....................................................................... 100/304
[01:22:13] .................................................................................................... 300/304
[01:22:15] ....
[01:22:15] failures:
[01:22:15] 
[01:22:15] 
[01:22:15] ---- [rustdoc] rustdoc/async-fn.rs stdout ----
[01:22:15] 
[01:22:15] error: htmldocck failed!
[01:22:15] status: exit code: 1
[01:22:15] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
[01:22:15] ------------------------------------------
[01:22:15] 
[01:22:15] ------------------------------------------
[01:22:15] stderr:
[01:22:15] stderr:
[01:22:15] ------------------------------------------
[01:22:15] 10: @has check failed
[01:22:15]  `XPATH PATTERN` did not match
[01:22:15]  // @has async_fn/fn.bar.html '//pre[@class="rust fn"]' 'pub async fn bar(a: i32, b: i32) -> i32'
[01:22:15] 15: @has check failed
[01:22:15]  `XPATH PATTERN` did not match
[01:22:15]  // @has async_fn/fn.baz.html '//pre[@class="rust fn"]' 'pub async fn baz<T>(a: T) -> T'
[01:22:15] Encountered 2 errors
[01:22:15] 
[01:22:15] ------------------------------------------
[01:22:15] 
---
[01:22:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:22:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:22:15] 
[01:22:15] 
[01:22:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:15] 
[01:22:15] 
[01:22:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:15] Build completed unsuccessfully in 0:19:36
[01:22:15] Build completed unsuccessfully in 0:19:36
[01:22:15] make: *** [check] Error 1
[01:22:15] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03394940
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 12:25:37 UTC 2019
---
travis_time:end:051f3bd6:start=1555849539230765653,finish=1555849539235367297,duration=4601644
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00667200
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ba4860
travis_time:start:04ba4860
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07dfe26a
$ dmesg | grep -i kill

plain
travis_time:end:077ae3bc:start=1557701249335574718,finish=1557701250128264460,duration=792689742
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:44] 
[01:16:44] running 2960 tests
[01:16:57] ......F............................................................................................. 100/2960
[01:17:19] .................................................................................................... 300/2960
[01:17:31] .................................................................................................... 400/2960
[01:17:41] .................................................................................................... 500/2960
[01:17:53] .................................................................................................... 600/2960
---
[01:23:37] failures:
[01:23:37] 
[01:23:37] ---- [run-pass] run-pass/allocator-alloc-one.rs stdout ----
[01:23:37] 
[01:23:37] error: test compilation failed although it shouldn't!
[01:23:37] status: exit code: 1
[01:23:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator-alloc-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator-alloc-one/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator-alloc-one/auxiliary"
[01:23:37] ------------------------------------------
[01:23:37] 
[01:23:37] ------------------------------------------
[01:23:37] stderr:
[01:23:37] stderr:
[01:23:37] ------------------------------------------
[01:23:37] warning: unused import: `Alloc`
[01:23:37]   --> /checkout/src/test/run-pass/allocator-alloc-one.rs:5:18
[01:23:37]    |
[01:23:37] LL | use std::alloc::{Alloc, Global, Layout, handle_alloc_error};
[01:23:37]    |
[01:23:37]    = note: #[warn(unused_imports)] on by default
[01:23:37] 
[01:23:37] error[E0599]: no method named `alloc_one` found for type `std::alloc::Global` in the current scope
---
[01:23:37] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:23:37] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:23:37] 
[01:23:37] 
[01:23:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:23:37] 
[01:23:37] 
[01:23:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:37] Build completed unsuccessfully in 0:11:46
[01:23:37] Build completed unsuccessfully in 0:11:46
[01:23:37] Makefile:48: recipe for target 'check' failed
[01:23:37] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bea33ff
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 13 00:11:18 UTC 2019
---
travis_time:end:273b4dcc:start=1557706279668799713,finish=1557706279673916156,duration=5116443
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06150a58
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0425ec62
travis_time:start:0425ec62
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1561f458
$ dmesg | grep -i kill

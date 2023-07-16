plain
travis_time:end:00157aa0:start=1552515438494384411,finish=1552515525729300107,duration=87234915696
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
[01:09:57] 
[01:09:57] running 5467 tests
[01:09:59] ..............................................................................................F..... 100/5467
[01:10:06] .................................................................................................... 300/5467
[01:10:09] .................................................................................................... 400/5467
[01:10:12] .................................................................................................... 500/5467
[01:10:16] ..........................................i......................................................... 600/5467
---
[01:12:51] ............i....................................................................................... 4700/5467
[01:12:57] .................................................................................................... 4800/5467
[01:13:00] .................................................................................................... 4900/5467
[01:13:04] .................................................................................................... 5000/5467
[01:13:08] ..................................F................................................................. 5100/5467
[01:13:14] .................................................................................................... 5300/5467
[01:13:17] .................................................................................................... 5400/5467
[01:13:19] .....i.............................................................
[01:13:19] failures:
[01:13:19] failures:
[01:13:19] 
[01:13:19] ---- [ui] ui/associated-types/associated-types-overridden-binding-2.rs stdout ----
[01:13:19] 
[01:13:19] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:13:19] status: exit code: 101
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding-2/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
---
[01:13:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:19] 
[01:13:19] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:13:19] 
[01:13:19] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] thread '[ui] ui/associated-types/associated-types-overridden-binding-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:13:19] thread '[ui] ui/associated-types/associated-types-overridden-binding-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:13:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:19] 
[01:13:19] ---- [ui] ui/traits/trait-alias-object.rs stdout ----
[01:13:19] 
[01:13:19] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:13:19] status: exit code: 101
[01:13:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/auxiliary" "-A" "unused"
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] stderr:
---
[01:13:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:13:19] 
[01:13:19] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[01:13:19] 
[01:13:19] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:13:19] 
[01:13:19] ------------------------------------------
[01:13:19] 
[01:13:19] thread '[ui] ui/traits/trait-alias-object.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:13:19] 
[01:13:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:13:19] 
[01:13:19] 
[01:13:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:19] 
[01:13:19] 
[01:13:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:19] Build completed unsuccessfully in 0:04:24
[01:13:19] Build completed unsuccessfully in 0:04:24
[01:13:19] Makefile:48: recipe for target 'check' failed
[01:13:19] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0006e7f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 13 23:32:14 UTC 2019

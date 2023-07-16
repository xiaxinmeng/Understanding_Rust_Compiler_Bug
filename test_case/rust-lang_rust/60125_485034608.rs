plain
travis_time:end:01e88006:start=1555710893056536810,finish=1555710895109158321,duration=2052621511
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:06] .................................................................................................... 500/5546
[01:09:11] ................................................i................................................... 600/5546
[01:09:15] .................................................................................................... 700/5546
[01:09:20] .................................................................................................... 800/5546
[01:09:25] .....................................................................F.............................. 900/5546
[01:09:34] .............................................iiiii.................................................. 1100/5546
[01:09:38] .................................................................................................... 1200/5546
[01:09:41] .................................................................................................... 1300/5546
[01:09:44] .................................................................................................... 1400/5546
---
[01:12:38] 20 
[01:12:38] - note: rustc 1.36.0-dev running on x86_64-apple-darwin
[01:12:38] + note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:12:38] 22 
[01:12:38] 23 note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:12:38] 
[01:12:38] 
[01:12:38] The actual stderr differed from the expected stderr.
[01:12:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/match_ice/match_ice.stderr
[01:12:38] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/match_ice/match_ice.stderr
[01:12:38] To update references, rerun the tests and pass the `--bless` flag
[01:12:38] To only update this specific test, also pass `--test-args consts/match_ice.rs`
[01:12:38] error: 1 errors occurred comparing output.
[01:12:38] status: exit code: 101
[01:12:38] status: exit code: 101
[01:12:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/match_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/match_ice/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/match_ice/auxiliary" "-A" "unused"
[01:12:38] ------------------------------------------
[01:12:38] 
[01:12:38] ------------------------------------------
[01:12:38] stderr:
[01:12:38] stderr:
[01:12:38] ------------------------------------------
[01:12:38] error[E0004]: non-exhaustive patterns: `&S` not covered
[01:12:38]    |
[01:12:38] LL |     match C { //~ ERROR non-exhaustive
[01:12:38] LL |     match C { //~ ERROR non-exhaustive
[01:12:38]    |           ^ pattern `&S` not covered
[01:12:38]    |
[01:12:38]    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
[01:12:38] 
[01:12:38] error: internal compiler error: src/librustc/traits/codegen/mod.rs:156: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<S as std::cmp::PartialEq>)),depth=1),Unimplemented)]` resolving bounds after type-checking
[01:12:38] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:571:9
[01:12:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:38] error: aborting due to 2 previous errors
[01:12:38] 
---
[01:12:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:38] 
[01:12:38] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:12:38] 
[01:12:38] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:12:38] 
[01:12:38] ------------------------------------------
[01:12:38] 
[01:12:38] 
---
[01:12:38] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:12:38] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:38] 
[01:12:38] 
[01:12:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:38] 
[01:12:38] 
[01:12:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:38] Build completed unsuccessfully in 0:05:00
[01:12:38] Build completed unsuccessfully in 0:05:00
[01:12:38] make: *** [check] Error 1
[01:12:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01277470
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 19 23:07:44 UTC 2019

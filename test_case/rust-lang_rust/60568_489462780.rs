plain
travis_time:end:02b5c1f4:start=1557085473552639684,finish=1557085474339711111,duration=787071427
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:54] ...iiiii............................................................................................ 1100/5493
[01:10:57] .................................................................................................... 1200/5493
[01:10:59] .................................................................................................... 1300/5493
[01:11:02] .................................................................................................... 1400/5493
[01:11:05] ........................F........................................................................... 1500/5493
[01:11:11] ..................i................................................................................. 1700/5493
[01:11:15] .................................................................................................... 1800/5493
[01:11:19] .................................................................................................... 1900/5493
[01:11:22] .................................................................................................... 2000/5493
---
[01:13:39] 
[01:13:39] - error: extern items cannot be `const`
[01:13:39] -   --> $DIR/extern-const.rs:15:5
[01:13:39] -    |
[01:13:39] - LL |     const rust_dbg_static_mut: libc::c_int;
[01:13:39] -    |     ^^^^^ help: try using a static value: `static`
[01:13:39] - error: aborting due to previous error
[01:13:39] - error: aborting due to previous error
[01:13:39] + error: unknown codegen option: `-Zunstable-options`
[01:13:39] 9 
[01:13:39] 
[01:13:39] 
[01:13:39] The actual stderr differed from the expected stderr.
[01:13:39] The actual stderr differed from the expected stderr.
[01:13:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/extern-const.stderr
[01:13:39] diff of fixed:
[01:13:39] 
[01:13:39] 12 
[01:13:39] 13 #[link(name = "rust_test_helpers", kind = "static")]
[01:13:39] 14 extern "C" {
[01:13:39] -     static rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
[01:13:39] +     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
[01:13:39] 17 
[01:13:39] 18 fn main() {
[01:13:39] 
[01:13:39] 
[01:13:39] 
[01:13:39] The actual fixed differed from the expected fixed.
[01:13:39] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/extern-const.fixed
[01:13:39] To update references, rerun the tests and pass the `--bless` flag
[01:13:39] To only update this specific test, also pass `--test-args extern/extern-const.rs`
[01:13:39] error: 2 errors occurred comparing output.
[01:13:39] status: exit code: 1
[01:13:39] status: exit code: 1
[01:13:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a" "-Crpath" "-O" "-C" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/auxiliary" "-A" "unused"
[01:13:39] ------------------------------------------
[01:13:39] 
[01:13:39] ------------------------------------------
[01:13:39] stderr:
[01:13:39] stderr:
[01:13:39] ------------------------------------------
[01:13:39] error: unknown codegen option: `-Zunstable-options`
[01:13:39] 
[01:13:39] ------------------------------------------
[01:13:39] 
[01:13:39] 
---
[01:13:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:13:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:39] 
[01:13:39] 
[01:13:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -C debuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -C debuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:39] 
[01:13:39] 
[01:13:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:39] Build completed unsuccessfully in 0:04:32
[01:13:39] Build completed unsuccessfully in 0:04:32
[01:13:39] make: *** [check] Error 1
[01:13:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24277d64
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May  5 20:58:25 UTC 2019

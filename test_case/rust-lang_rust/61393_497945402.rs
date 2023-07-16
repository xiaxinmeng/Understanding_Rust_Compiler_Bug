plain
travis_time:end:0a0d20f8:start=1559391771090533947,finish=1559391771857680938,duration=767146991
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:02:27] .................................................................................................... 1100/2923
[01:02:36] .................................................................................................... 1200/2923
[01:02:47] .................................................................................................... 1300/2923
[01:02:59] ....................ii.............................................................................. 1400/2923
[01:03:11] .......................................................F............................................ 1500/2923
[01:03:34] .................................................................................................... 1700/2923
[01:03:48] .................................................................................................... 1800/2923
[01:03:58] .................................................................................................... 1900/2923
[01:04:13] ..i.......................................................................i......................... 2000/2923
---
[01:06:53] failures:
[01:06:53] 
[01:06:53] ---- [run-pass] run-pass/issues/issue-3656.rs stdout ----
[01:06:53] normalized stderr:
[01:06:53] warning: use of deprecated item 'libc::uint32_t': Use u32 instead.
[01:06:53]    |
[01:06:53]    |
[01:06:53] LL | use libc::{c_uint, uint32_t, c_void};
[01:06:53]    |
[01:06:53]    = note: #[warn(deprecated)] on by default
[01:06:53] 
[01:06:53] 
[01:06:53] warning: use of deprecated item 'libc::uint32_t': Use u32 instead.
[01:06:53]    |
[01:06:53] LL |     count: uint32_t,
[01:06:53]    |            ^^^^^^^^
[01:06:53] 
[01:06:53] 
[01:06:53] warning: use of deprecated item 'libc::uint32_t': Use u32 instead.
[01:06:53]    |
[01:06:53]    |
[01:06:53] LL |     salt_size: uint32_t,
[01:06:53] 
[01:06:53] 
[01:06:53] 
[01:06:53] 
[01:06:53] 
[01:06:53] The actual stderr differed from the expected stderr.
[01:06:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3656/issue-3656.stderr
[01:06:53] To update references, rerun the tests and pass the `--bless` flag
[01:06:53] To only update this specific test, also pass `--test-args issues/issue-3656.rs`
[01:06:53] error: 1 errors occurred comparing output.
[01:06:53] status: exit code: 0
[01:06:53] status: exit code: 0
[01:06:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-3656.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3656/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-3656/auxiliary"
[01:06:53] ------------------------------------------
[01:06:53] 
[01:06:53] ------------------------------------------
[01:06:53] stderr:
[01:06:53] stderr:
[01:06:53] ------------------------------------------
[01:06:53] warning: use of deprecated item 'libc::uint32_t': Use u32 instead.
[01:06:53]    |
[01:06:53]    |
[01:06:53] LL | use libc::{c_uint, uint32_t, c_void};
[01:06:53]    |
[01:06:53]    = note: #[warn(deprecated)] on by default
[01:06:53] 
[01:06:53] 
[01:06:53] warning: use of deprecated item 'libc::uint32_t': Use u32 instead.
[01:06:53]    |
[01:06:53] LL |     count: uint32_t,
[01:06:53]    |            ^^^^^^^^
[01:06:53] 
[01:06:53] 
[01:06:53] warning: use of deprecated item 'libc::uint32_t': Use u32 instead.
[01:06:53]    |
[01:06:53]    |
[01:06:53] LL |     salt_size: uint32_t,
[01:06:53] 
[01:06:53] 
[01:06:53] ------------------------------------------
[01:06:53] 
---
[01:06:53] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:06:53] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:53] 
[01:06:53] 
[01:06:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:53] 
[01:06:53] 
[01:06:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:53] Build completed unsuccessfully in 1:02:02

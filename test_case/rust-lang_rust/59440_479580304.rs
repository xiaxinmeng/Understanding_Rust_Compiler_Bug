plain
travis_time:end:0ca62fc3:start=1554306805766345633,finish=1554306895935557756,duration=90169212123
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:28] 
######################################################################## 100.0%
[00:01:28] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:28]     Updating crates.io index
[00:01:49]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:01:50]   Downloaded libc v0.2.51
[00:01:50]   Downloaded serde_json v1.0.33
[00:01:50]   Downloaded time v0.1.40
[00:01:50]   Downloaded serde_derive v1.0.81
---
tidy check
[00:03:38] * 569 error codes
[00:03:38] * highest error code: E0725
[00:03:39] * 252 features
[00:03:39] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[00:03:39] travis_time:end:tidy:start=1554307123138909129,finish=1554307124958563724,duration=1819654595

[00:03:39] Build completed successfully in 0:00:46
---
[00:04:38]    Compiling libc v0.2.51
[00:04:38]    Compiling getopts v0.2.17
[00:04:38]    Compiling termcolor v1.0.4
[00:04:38]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:04:40]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:04:50]     Finished release [optimized] target(s) in 13.15s
[00:04:50] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:50] travis_fold:end:stage0-test

---
[00:27:47]    Compiling libc v0.2.51
[00:27:47]    Compiling termcolor v1.0.4
[00:27:47]    Compiling getopts v0.2.17
[00:27:47]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:27:50]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[00:28:03]     Finished release [optimized] target(s) in 15.71s
[00:28:03] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:28:03] travis_fold:end:stage1-test

---
[01:06:39]     Checking termcolor v1.0.4
[01:06:39]     Checking getopts v0.2.17
[01:06:39]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:06:39]     Checking libc v0.2.51
[01:06:40]     Checking libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
[01:06:45]     Finished release [optimized] target(s) in 6.95s
[01:06:45] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[01:06:46]  Documenting proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:06:49]     Finished release [optimized] target(s) in 4.04s
---
tidy check
[01:09:39] * 569 error codes
[01:09:39] * highest error code: E0725
[01:09:39] * 252 features
[01:09:40] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd286fd16c76bd61ddcbe162fca9f9d9d2"

[01:09:40] travis_time:end:tidy:start=1554311083626645044,finish=1554311085599965150,duration=1973320106

[01:09:40] travis_fold:start:stage0-std
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:37] 
[01:21:37] running 9 tests
[01:21:37] iiiiiiiii
[01:21:37] 
[01:21:37]  finished in 0.148
[01:21:37] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:53] 
[01:21:53] running 121 tests
[01:22:18] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:22:22] i.i......iii.i.....ii
[01:22:22] 
[01:22:22]  finished in 29.590
[01:22:22] travis_fold:end:test_debuginfo

---
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:23] 
[01:22:23] running 20 tests
[01:22:29] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:22:29] ...F................
[01:22:29] 
[01:22:29] ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
[01:22:29] 
[01:22:29] 
[01:22:29] error: /checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs:13: unexpected error: '13:10: 13:20: use of unstable library feature 'rustc_private': crate "rustc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812) [E0658]'
[01:22:29] error: 1 unexpected errors found, 0 expected errors not found
[01:22:29] status: exit code: 1
[01:22:29] status: exit code: 1
[01:22:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
[01:22:29]     Error {
[01:22:29]         line_num: 13,
[01:22:29]         kind: Some(
[01:22:29]             Error
[01:22:29]             Error
[01:22:29]         ),
[01:22:29]         msg: "13:10: 13:20: use of unstable library feature \'rustc_private\': crate \"rustc\" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812) [E0658]"
[01:22:29] ]
[01:22:29] 
[01:22:29] thread '[ui] ui-fulldeps/hash-stable-is-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1381:13
[01:22:29] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:22:29] test result: FAILED. 19 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:22:29] 
[01:22:29] 
[01:22:29] 
[01:22:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:29] 
[01:22:29] 
[01:22:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:29] Build completed unsuccessfully in 0:12:53
[01:22:29] Build completed unsuccessfully in 0:12:53
[01:22:29] Makefile:48: recipe for target 'check' failed
[01:22:29] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:091ad048
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 17:17:35 UTC 2019
---
travis_time:end:18bae43a:start=1554311856836462317,finish=1554311856892053447,duration=55591130
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e37aa30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0169fd57
$ dmesg | grep -i kill

plain
travis_time:end:0843304c:start=1555522398370705725,finish=1555522399467573317,duration=1096867592
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
[01:14:51] 
[01:14:51] running 9 tests
[01:14:51] iiiiiiiii
[01:14:51] 
[01:14:51]  finished in 0.164
[01:14:51] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:08] 
[01:15:08] running 121 tests
[01:15:36] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:15:42] i.i......iii.i.....ii
[01:15:42] 
[01:15:42]  finished in 33.849
[01:15:42] travis_fold:end:test_debuginfo

---
[01:15:42] 
[01:15:42] running 22 tests
[01:15:50] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:15:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:15:50] ....F..............F..
[01:15:50] 
[01:15:50] ---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
[01:15:50] diff of stderr:
[01:15:50] 
[01:15:50] 
[01:15:50] 49 
[01:15:50] 50 error: aborting due to 6 previous errors
[01:15:50] 51 
[01:15:50] - Some errors occurred: E0601, E0658.
[01:15:50] + Some errors have detailed explanations: E0601, E0658.
[01:15:50] 54 
[01:15:50] 
[01:15:50] 
[01:15:50] The actual stderr differed from the expected stderr.
[01:15:50] The actual stderr differed from the expected stderr.
[01:15:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
[01:15:50] To update references, rerun the tests and pass the `--bless` flag
[01:15:50] To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`
[01:15:50] error: 1 errors occurred comparing output.
[01:15:50] status: exit code: 1
[01:15:50] status: exit code: 1
[01:15:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary" "-A" "unused"
[01:15:50] ------------------------------------------
[01:15:50] 
[01:15:50] ------------------------------------------
[01:15:50] stderr:
[01:15:50] stderr:
[01:15:50] ------------------------------------------
[01:15:50] error[E0601]: `main` function not found in crate `hash_stable_is_unstable`
[01:15:50]    |
[01:15:50]    = note: consider adding a `main` function to `/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs`
[01:15:50] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
[01:15:50]   --> /checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs:3:1
[01:15:50]    |
[01:15:50] LL | extern crate rustc_data_structures;
---
[01:15:50] 
[01:15:50] error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
[01:15:50]   --> /checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs:13:10
[01:15:50]    |
[01:15:50] LL | #[derive(HashStable)]
[01:15:50]    |
[01:15:50]    = note: for more information, see https://github.com/rust-lang/rust/issues/27812
[01:15:50]    = help: add #![feature(rustc_private)] to the crate attributes to enable
[01:15:50] 
---
[01:15:50] 10 
[01:15:50] 
[01:15:50] 
[01:15:50] The actual stderr differed from the expected stderr.
[01:15:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib/macro-crate-rlib.stderr
[01:15:50] To update references, rerun the tests and pass the `--bless` flag
[01:15:50] To only update this specific test, also pass `--test-args macro-crate-rlib.rs`
[01:15:50] error: 1 errors occurred comparing output.
[01:15:50] status: exit code: 1
[01:15:50] status: exit code: 1
[01:15:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/macro-crate-rlib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/macro-crate-rlib/auxiliary" "-A" "unused"
[01:15:50] ------------------------------------------
[01:15:50] 
[01:15:50] ------------------------------------------
[01:15:50] stderr:
[01:15:50] stderr:
[01:15:50] ------------------------------------------
[01:15:50] error[E0457]: plugin `rlib_crate_test` only found in rlib format, but must be available in dylib format
[01:15:50]    |
[01:15:50]    |
[01:15:50] LL | #![plugin(rlib_crate_test)]
[01:15:50] 
[01:15:50] error: aborting due to previous error
[01:15:50] 
[01:15:50] 
---
[01:15:50] test result: FAILED. 20 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:15:50] 
[01:15:50] 
[01:15:50] 
[01:15:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:50] 
[01:15:50] 
[01:15:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:50] Build completed unsuccessfully in 0:13:21
[01:15:50] Build completed unsuccessfully in 0:13:21
[01:15:50] Makefile:48: recipe for target 'check' failed
[01:15:50] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cc2bdfc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 17 18:49:34 UTC 2019
---
travis_time:end:00ed2916:start=1555526976803274022,finish=1555526976861656309,duration=58382287
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09626ad4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18dba0ac
$ dmesg | grep -i kill

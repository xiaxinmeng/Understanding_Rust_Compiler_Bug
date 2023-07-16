plain
travis_time:end:13be2aa4:start=1559735068870919594,finish=1559735069933984804,duration=1063065210
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:11] 
[01:06:11] running 144 tests
[01:06:14] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:06:16] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:06:16] 
[01:06:16]  finished in 4.695
[01:06:16] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:18] 
[01:06:18] running 9 tests
[01:06:18] iiiiiiiii
[01:06:18] 
[01:06:18]  finished in 0.154
[01:06:18] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:34] 
[01:06:34] running 122 tests
[01:06:59] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:04] .i.i......iii.i.....ii
[01:07:04] 
[01:07:04]  finished in 30.327
[01:07:04] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:12] 
[01:07:12] running 48 tests
[01:08:40] ..........................................F....test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:10:54] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:10:54] .
[01:10:54] failures:
[01:10:54] 
[01:10:54] 
[01:10:54] ---- [run-pass] run-pass-fulldeps/roman-numerals-macro.rs stdout ----
[01:10:54] 
[01:10:54] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
[01:10:54] status: exit code: 1
[01:10:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/roman-numerals-macro/auxiliary"
[01:10:54] ------------------------------------------
[01:10:54] 
[01:10:54] ------------------------------------------
[01:10:54] stderr:
[01:10:54] stderr:
[01:10:54] ------------------------------------------
[01:10:54] warning: trait objects without an explicit `dyn` are deprecated
[01:10:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/roman-numerals.rs:25:16
[01:10:54]    |
[01:10:54] LL |         -> Box<MacResult + 'static> {
[01:10:54]    |                ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn MacResult + 'static`
[01:10:54]    = note: #[warn(bare_trait_objects)] on by default
[01:10:54] 
[01:10:54] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
[01:10:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/roman-numerals.rs:41:9
[01:10:54]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/roman-numerals.rs:41:9
[01:10:54]    |
[01:10:54] LL |         TokenTree::Token(_, token::Ident(s, _)) => s.to_string(),
[01:10:54] 
[01:10:54] error: aborting due to previous error
[01:10:54] 
[01:10:54] For more information about this error, try `rustc --explain E0023`.
---
[01:10:54] test result: FAILED. 47 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:10:54] 
[01:10:54] 
[01:10:54] 
[01:10:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:54] 
[01:10:54] 
[01:10:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:54] Build completed unsuccessfully in 1:05:55
---
31116 ./obj/build/xtravis_time:end:04b703a0:start=1559739337247133428,finish=1559739337302282559,duration=55149131
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09ec83e3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d2af1c6
$ dmesg | grep -i kill

plain
travis_time:end:09cdf074:start=1558895667459729582,finish=1558895669958638198,duration=2498908616
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
[01:21:13] 
[01:21:13] running 143 tests
[01:21:15] i..iii.....iii..i.iii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:21:17] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:21:17] 
[01:21:17]  finished in 4.684
[01:21:17] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:19] 
[01:21:19] running 9 tests
[01:21:19] iiiiiiiii
[01:21:19] 
[01:21:19]  finished in 0.150
[01:21:19] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:36] 
[01:21:36] running 122 tests
[01:22:00] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:22:05] .i.i......iii.i.....ii
[01:22:05] 
[01:22:05]  finished in 29.727
[01:22:05] travis_fold:end:test_debuginfo

---
[01:26:00] failures:
[01:26:00] 
[01:26:00] ---- [run-pass] run-pass-fulldeps/pprust-expr-roundtrip.rs stdout ----
[01:26:00] normalized stderr:
[01:26:00] warning: trait objects without an explicit `dyn` are deprecated
[01:26:00]   --> $DIR/pprust-expr-roundtrip.rs:65:37
[01:26:00]    |
[01:26:00] LL | fn iter_exprs(depth: usize, f: &mut FnMut(P<Expr>)) {
[01:26:00]    |                                     ^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(P<Expr>)`
[01:26:00]    |
[01:26:00]    = note: #[warn(bare_trait_objects)] on by default
[01:26:00] 
[01:26:00] 
[01:26:00] 
[01:26:00] The actual stderr differed from the expected stderr.
[01:26:00] The actual stderr differed from the expected stderr.
[01:26:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/pprust-expr-roundtrip.stderr
[01:26:00] To update references, rerun the tests and pass the `--bless` flag
[01:26:00] To only update this specific test, also pass `--test-args pprust-expr-roundtrip.rs`
[01:26:00] error: 1 errors occurred comparing output.
[01:26:00] status: exit code: 0
[01:26:00] status: exit code: 0
[01:26:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/pprust-expr-roundtrip/auxiliary"
[01:26:00] ------------------------------------------
[01:26:00] 
[01:26:00] ------------------------------------------
[01:26:00] stderr:
[01:26:00] stderr:
[01:26:00] ------------------------------------------
[01:26:00] warning: trait objects without an explicit `dyn` are deprecated
[01:26:00]    |
[01:26:00]    |
[01:26:00] LL | fn iter_exprs(depth: usize, f: &mut FnMut(P<Expr>)) {
[01:26:00]    |                                     ^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(P<Expr>)`
[01:26:00]    |
[01:26:00]    = note: #[warn(bare_trait_objects)] on by default
[01:26:00] 
[01:26:00] ------------------------------------------
[01:26:00] 
[01:26:00] 
---
[01:26:00] test result: FAILED. 47 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:26:00] 
[01:26:00] 
[01:26:00] 
[01:26:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:26:00] 
[01:26:00] 
[01:26:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:00] Build completed unsuccessfully in 0:16:48
[01:26:00] Build completed unsuccessfully in 0:16:48
[01:26:00] make: *** [check] Error 1
[01:26:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1dd572e5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 26 20:00:41 UTC 2019

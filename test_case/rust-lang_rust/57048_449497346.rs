plain
travis_time:end:0d0ea7fd:start=1545420625983944096,finish=1545420699002662928,duration=73018718832
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:59] 
[01:03:59] running 118 tests
[01:04:21] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:04:25] ......iii.i.....ii
[01:04:25] 
[01:04:25]  finished in 26.621
[01:04:25] travis_fold:end:test_debuginfo

---
[01:29:34] 
[01:29:34] failures:
[01:29:34] 
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7180) stdout ----
[01:29:34] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7188:5
[01:29:34]    |
[01:29:34] 10 |     fn x86_mm_movemask_pd<T>(x: f64x2) -> i32;
[01:29:34] 
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7180)' panicked at 'Some expected error codes were not found: ["E0440"]', src/librustdoc/test.rs:331:9
[01:29:34] 
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7197) stdout ----
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7197) stdout ----
[01:29:34] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7205:5
[01:29:34]    |
[01:29:34] 10 |     fn x86_mm_movemask_pd(x: f64x2) -> i32;
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7197)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:29:34] 
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0441 (line 7231) stdout ----
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0441 (line 7231) stdout ----
[01:29:34] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7239:5
[01:29:34]    |
[01:29:34] 10 |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i16x8; // ok!
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0441 (line 7231)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:29:34] 
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7248) stdout ----
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7248) stdout ----
[01:29:34] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7261:5
[01:29:34]    |
[01:29:34] 15 |     fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;
[01:29:34] 
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7248)' panicked at 'Some expected error codes were not found: ["E0442"]', src/librustdoc/test.rs:331:9
[01:29:34] 
[01:29:34] ---- /checkout/oction: `x86_mm_adds_epi16`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7312:5
[01:29:34]    |
[01:29:34] 10 |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i16x8; // ok!
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7304)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:29:34] 
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7321) stdout ----
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7321) stdout ----
[01:29:34] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7329:5
[01:29:34]    |
[01:29:34] 10 |     fn x86_mm_movemask_pd(x: f64x2, y: f64x2, z: f64x2) -> i32;
[01:29:34] 
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7321)' panicked at 'Some expected error codes were not found: ["E0444"]', src/librustdoc/test.rs:331:9
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7337) stdout ----
[01:29:34] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7337) stdout ----
[01:29:34] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:29:34]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7345:5
[01:29:34]    |
[01:29:34] 10 |     fn x86_mm_movemask_pd(x: f64x2) -> i32; // ok!
[01:29:34] 
[01:29:34] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7337)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:29:34] 
[01:29:34] 
---
[01:29:34] 
[01:29:34] 
[01:29:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:34] Build completed unsuccessfully in 0:36:14
[01:29:34] make: *** [check] Error 1
[01:29:34] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:025e539c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 21:01:22 UTC 2018
---
travis_time:end:07820d78:start=1545426085335545838,finish=1545426085341159379,duration=5613541
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01703504
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3e40dfda
travis_time:start:3e40dfda
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0293639d
$ dmesg | grep -i kill

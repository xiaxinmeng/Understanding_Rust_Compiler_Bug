plain
travis_time:end:03779520:start=1546889285199012584,finish=1546889286327383863,duration=1128371279
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
[01:07:07] 
[01:07:07] running 118 tests
[01:07:31] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:07:35] ......iii.i.....ii
[01:07:35] 
[01:07:35]  finished in 28.214
[01:07:35] travis_fold:end:test_debuginfo

---
[01:33:59] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0692 (line 11303) ... ok
[01:33:59] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0699 (line 11389) ... ignored
[01:33:59] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0695 (line 11335) ... ok
[01:33:59] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0701 (line 11457) ... ok
[01:33:59] test /checkout/oknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7197) stdout ----
[01:33:59] warning: type `f64x2` should have a camel case name such as `F64x2`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7202:1
[01:33:59]   |
[01:33:59] 7 | struct f64x2(f64, f64);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7205:5
[01:33:59]    |
[01:33:59] 10 |     fn x86_mm_movemask_pd(x: f64x2) -> i32;
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0440 (line 7197)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:33:59] 
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0441 (line 7231) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0441 (line 7231) stdout ----
[01:33:59] warning: type `i16x8` should have a camel case name such as `I16x8`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7236:1
[01:33:59]   |
[01:33:59] 7 | struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7239:5
[01:33:59]    |
[01:33:59] 10 |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i16x8; // ok!
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0441 (line 7231)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:33:59] 
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7269) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7269) stdout ----
[01:33:59] warning: type `i16x8` should have a camel case name such as `I16x8`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7274:1
[01:33:59]   |
[01:33:59] 7 | struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7277:5
[01:33:59]    |
[01:33:59] 10 |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i16x8; // ok!
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7269)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:33:59] 
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7248) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7248) stdout ----
[01:33:59] warning: type `i8x16` should have a camel case name such as `I8x16`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7253:1
[01:33:59]   |
[01:33:59] 7 | / struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8,
[01:33:59] 8 | |              i8, i8, i8, i8, i8, i8, i8, i8);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] warning: type `i32x4` should have a camel case name such as `I32x4`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7256:1
[01:33:59]    |
[01:33:59] 10 | struct i32x4(i32, i32, i32, i32);
[01:33:59] 
[01:33:59] 
[01:33:59] warning: type `i64x2` should have a camel case name such as `I64x2`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7258:1
[01:33:59]    |
[01:33:59] 12 | struct i64x2(i64, i64);
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7261:5
[01:33:59]    |
[01:33:59] 15 |     fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;
[01:33:59] 
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0442 (line 7248)' panicked at 'Some expected error codes were not found: ["E0442"]', src/librustdoc/test.rs:326:9
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7304) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7304) stdout ----
[01:33:59] warning: type `i16x8` should have a camel case name such as `I16x8`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7309:1
[01:33:59]   |
[01:33:59] 7 | struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7312:5
[01:33:59]    |
[01:33:59] 10 |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i16x8; // ok!
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7304)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:33:59] 
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7286) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7286) stdout ----
[01:33:59] warning: type `i16x8` should have a camel case name such as `I16x8`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7291:1
[01:33:59]   |
[01:33:59] 7 | struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] warning: type `i64x8` should have a camel case name such as `I64x8`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7293:1
[01:33:59]   |
[01:33:59] 9 | struct i64x8(i64, i64, i64, i64, i64, i64, i64, i64);
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_adds_epi16`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7296:5
[01:33:59]    |
[01:33:59] 12 |     fn x86_mm_adds_epi16(x: i16x8, y: i16x8) -> i64x8;
[01:33:59] 
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0443 (line 7286)' panicked at 'Some expected error codes were not found: ["E0443"]', src/librustdoc/test.rs:326:9
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7321) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7321) stdout ----
[01:33:59] warning: type `f64x2` should have a camel case name such as `F64x2`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7326:1
[01:33:59]   |
[01:33:59] 7 | struct f64x2(f64, f64);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7329:5
[01:33:59]    |
[01:33:59] 10 |     fn x86_mm_movemask_pd(x: f64x2, y: f64x2, z: f64x2) -> i32;
[01:33:59] 
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7321)' panicked at 'Some expected error codes were not found: ["E0444"]', src/librustdoc/test.rs:326:9
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7337) stdout ----
[01:33:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7337) stdout ----
[01:33:59] warning: type `f64x2` should have a camel case name such as `F64x2`
[01:33:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7342:1
[01:33:59]   |
[01:33:59] 7 | struct f64x2(f64, f64);
[01:33:59]   |
[01:33:59]   = note: #[warn(non_camel_case_types)] on by default
[01:33:59] 
[01:33:59] 
[01:33:59] error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_movemask_pd`
[01:33:59]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7345:5
[01:33:59]    |
[01:33:59] 10 |     fn x86_mm_movemask_pd(x: f64x2) -> i32; // ok!
[01:33:59] 
[01:33:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0444 (line 7337)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:33:59] 
[01:33:59] 
---
[01:33:59] 
[01:33:59] 
[01:33:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:59] Build completed unsuccessfully in 0:38:01
[01:33:59] Makefile:48: recipe for target 'check' failed
[01:33:59] make: *** [check] Error 1
134116 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
127404 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
127400 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
124452 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps

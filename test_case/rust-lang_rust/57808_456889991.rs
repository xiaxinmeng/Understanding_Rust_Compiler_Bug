plain
travis_time:end:021c86a6:start=1548258600206082521,finish=1548258693623223753,duration=93417141232
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
[01:12:13] 
[01:12:13] running 118 tests
[01:12:37] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:12:40] ......iii.i.....ii
[01:12:40] 
[01:12:40]  finished in 27.674
[01:12:40] travis_fold:end:test_debuginfo

---
[01:24:32] 
[01:24:32]    Doc-tests core
[01:24:38] 
[01:24:38] running 2240 tests
[01:24:48] FFFFiFFiiiiiiF...................................................................................... 100/2240
[01:25:13] .................................................................................................... 300/2240
[01:25:27] ..........................................................i......................................... 400/2240
[01:25:39] .................................................................................................... 500/2240
[01:25:50] .................................................................................................... 600/2240
---
[01:29:08]    |
[01:29:08] 10 | extern crate std_detect;
[01:29:08]    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:29:08] 
[01:29:08] thread '../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m256 (line 292)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:29:08] 
[01:29:08] ---- ../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m128i (line 299) stdout ----
[01:29:08] error[E0463]: can't find crate for `std_detect`
[01:29:08]   --> ../stdsimd/crates/core_arch/src/macros.rs:307:1
[01:29:08]   --> ../stdsimd/crates/core_arch/src/macros.rs:307:1
[01:29:08]    |
[01:29:08] 10 | extern crate std_detect;
[01:29:08]    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:29:08] 
[01:29:08] thread '../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m128i (line 299)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:29:08] ---- ../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m128d (line 292) stdout ----
[01:29:08] error[E0463]: can't find crate for `std_detect`
[01:29:08]   --> ../stdsimd/crates/core_arch/src/macros.rs:300:1
[01:29:08]    |
[01:29:08]    |
[01:29:08] 10 | extern crate std_detect;
[01:29:08]    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:29:08] 
[01:29:08] thread '../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m128d (line 292)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:29:08] ---- ../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m128 (line 292) stdout ----
[01:29:08] error[E0463]: can't find crate for `std_detect`
[01:29:08]   --> ../stdsimd/crates/core_arch/src/macros.rs:300:1
[01:29:08]    |
[01:29:08]    |
[01:29:08] 10 | extern crate std_detect;
[01:29:08]    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:29:08] 
[01:29:08] thread '../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m128 (line 292)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:29:08] ---- ../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m256d (line 292) stdout ----
[01:29:08] error[E0463]: can't find crate for `std_detect`
[01:29:08]   --> ../stdsimd/crates/core_arch/src/macros.rs:300:1
[01:29:08]    |
[01:29:08]    |
[01:29:08] 10 | extern crate std_detect;
[01:29:08]    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:29:08] 
[01:29:08] thread '../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m256d (line 292)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:29:08] ---- ../stdsimd/crates/core_arch/src/macros.rs - core_arch::x86::__m256i (line 296) stdout ----
[01:29:08] error[E0463]: can't find crate for `std_detect`
[01:29:08]   --> ../stdsimd/crates/core_arch/src/macros.rs:304:1
[01:29:08]    |
[01:29:08]    |
[01:29:08] 10 | extern crate std_detect;
[01:29:08]    | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:29:08] 
[01:29:08] thread '..execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:29:08] 
[01:29:08] 
[01:29:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:08] Build completed unsuccessfully in 0:28:28
[01:29:08] Build completed unsuccessfully in 0:28:28
[01:29:08] make: *** [check] Error 1
[01:29:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2a97df58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 23 17:20:50 UTC 2019

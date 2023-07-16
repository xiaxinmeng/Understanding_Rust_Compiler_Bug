plain
travis_time:end:1e8413b0:start=1541510648620989342,finish=1541510840203116873,duration=191582127531
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:55:08] .................................................................................................... 100/4995
[00:55:11] .................................................................................................... 200/4995
[00:55:14] ........................................................................ii....................ii.... 300/4995
[00:55:17] ...........................................................................................iii...... 400/4995
[00:55:20] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4995
[00:55:27] .................................................................................................... 700/4995
[00:55:33] ...................................................................i...........i.................... 800/4995
[00:55:36] ......................................................................................iiiii......... 900/4995
[00:55:40] .........ii.iiii.................................................................................... 1000/4995
---
[00:56:15] .................................................................................................... 2200/4995
[00:56:20] .................................................................................................... 2300/4995
[00:56:23] .................................................................................................... 2400/4995
[00:56:27] .................................................................................................... 2500/4995
[00:56:31] ............................................................................iiiiiiiii............... 2600/4995
[00:56:38] ...........................ii....................................................................... 2800/4995
[00:56:40] .................................................................................................... 2900/4995
[00:56:44] .................................................................................................... 3000/4995
[00:56:47] ......................i............................................................................. 3100/4995
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:56] 
[01:10:56] running 115 tests
[01:10:59] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii...............i..ii..ii 100/115
[01:10:59] .i....iiii.....
[01:10:59] 
[01:10:59]  finished in 3.533
[01:10:59] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:14] 
[01:11:14] running 118 tests
[01:11:37] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:11:41] ......iii.i.....ii
[01:11:41] 
[01:11:41]  finished in 27.275
[01:11:41] travis_fold:end:test_debuginfo

---
[01:26:28] .................................................................................................... 200/2194
[01:26:41] .................................................................................................... 300/2194
[01:26:54] ..............................................i..................................................... 400/2194
[01:27:05] .................................................................................................... 500/2194
[01:27:15] .......F................................F.....................................F..................... 600/2194
[01:27:25] ...........F......................................F...............................F................. 700/2194
[01:27:36] ......................F.............................F.......................................F....... 800/2194
[01:27:47] ........................F.......................................F..............................F.... 900/2194
[01:27:58] ...............................F...............................F.................................... 1000/2194
[01:28:08] F..............................F....................................F.............................F. 1100/2194
[01:28:19] ...................................F...............................F................................ 1200/2194
[01:28:30] ................F...................................F...................................F........... 1300/2194
[01:28:41] ...................F................................................................................ 1400/2194
[01:29:02] .................................................................................................... 1600/2194
[01:29:13] .................................................................................................... 1700/2194
[01:29:25] .................................................................................................... 1800/2194
[01:29:38] .................................................................................................... 1900/2194
---
[01:30:20] ---- num/mod.rs - num::i128::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 1);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i128::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i128::trailing_ones (line 45) stdout ----
[01:30:20] ---- num/mod.rs - num::i128::trailing_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i128::trailing_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i16::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 1);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i16::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i16::trailing_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i16::trailing_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i32::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 1);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i32::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i32::trailing_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i32::trailing_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i64::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 1);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i64::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i64::trailing_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i64::trailing_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i8::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 1);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i8::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::i8::trailing_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::i8::trailing_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::isize::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 1);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::isize::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::isize::trailing_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::isize::trailing_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u128::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u128::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u128::trailing_ones (line 46) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:49:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u128::trailing_ones (line 46)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u16::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u16::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u16::trailing_ones (line 46) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:49:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u16::trailing_ones (line 46)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u32::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u32::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u32::trailing_ones (line 46) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:49:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u32::trailing_ones (line 46)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u64::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u64::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u64::trailing_ones (line 46) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:49:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u64::trailing_ones (line 46)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u8::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u8::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::u8::trailing_ones (line 46) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:49:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::u8::trailing_ones (line 46)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::usize::leading_ones (line 45) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:48:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.leading_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::usize::leading_ones (line 45)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] ---- num/mod.rs - num::usize::trailing_ones (line 46) stdout ----
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20] error[E0658]: use of unstable library feature 'leading_ones_trailing_ones'
[01:30:20]  --> num/mod.rs:49:14
[01:30:20]   |
[01:30:20] 6 | assert_eq!(n.trailing_ones(), 2);
[01:30:20]   |
[01:30:20]   |
[01:30:20]   = help: add #![feature(leading_ones_trailing_ones)] to the crate attributes to enable
[01:30:20] thread 'num/mod.rs - num::usize::trailing_ones (line 46)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:30:20] 
[01:30:20] 
[01:30:20] failures:
---
[01:30:20] 
[01:30:20] error: test failed, to rerun pass '--doc'
[01:30:20] 
[01:30:20] 
[01:30:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:30:20] 
[01:30:20] 
[01:30:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:20] Build completed unsuccessfully in 0:38:59
[01:30:20] Build completed unsuccessfully in 0:38:59
[01:30:20] make: *** [check] Error 1
[01:30:20] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:055fe321
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04fa4b4c:start=1541516273128371973,finish=1541516273132975233,duration=4603260
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12bcc87c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10dae0d0
travis_time:start:10dae0d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d1a7b8d
$ dmesg | grep -i kill

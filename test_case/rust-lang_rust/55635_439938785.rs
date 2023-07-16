plain
travis_time:end:02025acc:start=1542638088360644539,finish=1542638150703383508,duration=62342738969
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:07]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:07]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:08]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:08]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     |
[00:04:15]     |
[00:04:15] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:15]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:15] ...
[00:04:15] 110 | / nonzero_integers! {
[00:04:15] 111 | |     NonZeroU8(u8);
[00:04:15] 112 | |     NonZeroU16(u16);
[00:04:15] 113 | |     NonZeroU32(u32);
[00:04:15] 116 | |     NonZeroUsize(usize);
[00:04:15] 117 | | }
[00:04:15]     | |_- in this macro invocation
[00:04:15]     |
[00:04:15]     |
[00:04:15]     = note: #[warn(unused_unsafe)] on by default
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     |
[00:04:15]     |
[00:04:15] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:15]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:15] ...
[00:04:15] 110 | / nonzero_integers! {
[00:04:15] 111 | |     NonZeroU8(u8);
[00:04:15] 112 | |     NonZeroU16(u16);
[00:04:15] 113 | |     NonZeroU32(u32);
[00:04:15] 116 | |     NonZeroUsize(usize);
[00:04:15] 117 | | }
[00:04:15]     | |_- in this macro invocation
[00:04:15] 
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     |
[00:04:15]     |
[00:04:15] 81  |                           Some($Ty(unsafe { NonZero(n) }))
[00:04:15]     |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:15] ...
[00:04:15] 110 | / nonzero_integers! {
[00:04:15] 111 | |     NonZeroU8(u8);
[00:04:15] 112 | |     NonZeroU16(u16);
[00:04:15] 113 | |     NonZeroU32(u32);
[00:04:15] 116 | |     NonZeroUsize(usize);
[00:04:15] 117 | | }
[00:04:15]     | |_- in this macro invocation
[00:04:15] 
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     --> libcore/ptr.rs:2755:36
[00:04:15]      |
[00:04:15] 2755 |             Some(Unique { pointer: unsafe { NonZero(ptr as _) }, _marker: PhantomData })
[00:04:15]      |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     --> libcore/ptr.rs:2811:27
[00:04:15]      |
[00:04:15] 2811 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:15]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     --> libcore/ptr.rs:2818:27
[00:04:15]      |
[00:04:15] 2818 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:15]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     --> libcore/ptr.rs:2891:28
[00:04:15]      |
[00:04:15] 2891 |         NonNull { pointer: unsafe { NonZero(ptr as _) } }
[00:04:15]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     --> libcore/ptr.rs:3021:28
[00:04:15]      |
[00:04:15] 3021 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:15]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]     --> libcore/ptr.rs:3029:28
[00:04:15]      |
[00:04:15] 3029 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:15]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:15] 
[00:04:15] warning: unnecessary `unsafe` block
[00:04:15]    |
[00:04:15]    |
[00:04:15] 27 |         unsafe { NonZero(self.0) }
[00:04:15]    |         ^^^^^^ unnecessary `unsafe` block
[00:04:22]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:04:22]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:23]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:27]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
---
[00:49:27] .................................................................................................... 100/5047
[00:49:30] .................................................................................................... 200/5047
[00:49:33] .............................ii............................................ii...................ii.. 300/5047
[00:49:35] ..............................................................................................iii... 400/5047
[00:49:38] .....iiiiiiii.iii............................iii...........................................i........ 500/5047
[00:49:45] .................................................................................................... 700/5047
[00:49:50] .....................................................................................i...........i.. 800/5047
[00:49:54] .................................................................................................... 900/5047
[00:49:57] ....iiiii...................iiiiii.................................................................. 1000/5047
---
[00:50:31] .................................................................................................... 2200/5047
[00:50:35] .................................................................................................... 2300/5047
[00:50:39] .................................................................................................... 2400/5047
[00:50:43] .................................................................................................... 2500/5047
[00:50:46] ........................................................................................iiiiiiiii... 2600/5047
[00:50:53] ......................................................ii............................................ 2800/5047
[00:50:55] .................................................................................................... 2900/5047
[00:50:59] .................................................................................................... 3000/5047
[00:51:02] .................................................i.................................................. 3100/5047
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:15] 
[01:04:15] running 116 tests
[01:04:18] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:04:19] i.i....iiii.....
[01:04:19] 
[01:04:19]  finished in 3.419
[01:04:19] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:33] 
[01:04:33] running 118 tests
[01:04:57] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:01] ......iii.i.....ii
[01:05:01] 
[01:05:01]  finished in 28.056
[01:05:01] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:39] 
[01:05:39] running 97 tests
[01:07:39] ........................................F.................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:10:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:10:22] failures:
[01:10:22] 
[01:10:22] ---- [run-pass] run-pass-fulldeps/newtype_index.rs stdout ----
[01:10:22] normalized stderr:
[01:10:22] normalized stderr:
[01:10:22] warning: unused imports: `Decodable`, `Decoder`
[01:10:22]    |
[01:10:22]    |
[01:10:22] LL | use rustc_serialize::{Decodable, Decoder};
[01:10:22]    |
[01:10:22]    = note: #[warn(unused_imports)] on by default
[01:10:22] 
[01:10:22] 
[01:10:22] 
[01:10:22] 
[01:10:22] 
[01:10:22] The actual stderr differed from the expected stderr.
[01:10:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/newtype_index.stderr
[01:10:22] To update references, rerun the tests and pass the `--bless` flag
[01:10:22] To only update this specific test, also pass `--test-args newtype_index.rs`
[01:10:22] error: 1 errors occurred comparing output.
[01:10:22] status: exit code: 0
[01:10:22] status: exit code: 0
[01:10:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/newtype_index.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/newtype_index/auxiliary"
[01:10:22] ------------------------------------------
[01:10:22] 
[01:10:22] ------------------------------------------
[01:10:22] stderr:
[01:10:22] stderr:
[01:10:22] ------------------------------------------
[01:10:22] {"message":"unused imports: `Decodable`, `Decoder`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/newtype_index.rs","byte_start":175,"byte_end":184,"line_start":5,"line_end":5,"column_start":23,"column_end":32,"is_primary":true,"text":[{"text":"use rustc_serialize::{Decodable, Decoder};","highlight_start":23,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass-fulldeps/newtype_index.rs","byte_start":186,"byte_end":193,"line_start":5,"line_end":5,"column_start":34,"column_end":41,"is_primary":true,"text":[{"text":"use rustc_serialize::{Decodable, Decoder};","highlight_start":34,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_imports)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused imports: `Decodable`, `Decoder`\n  --> /checkout/src/test/run-pass-fulldeps/newtype_index.rs:5:23\n   |\nLL | use rustc_serialize::{Decodable, Decoder};\n   |                       ^^^^^^^^^  ^^^^^^^\n   |\n   = note: #[warn(unused_imports)] on by default\n\n"}
[01:10:22] ------------------------------------------
[01:10:22] 
[01:10:22] thread '[run-pass] run-pass-fulldeps/newtype_index.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:10:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:10:22] test result: FAILED. 96 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:10:22] 
[01:10:22] 
[01:10:22] 
[01:10:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:22] 
[01:10:22] 
[01:10:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:22] Build completed unsuccessfully in 0:24:31
[01:10:22] Build completed unsuccessfully in 0:24:31
[01:10:22] Makefile:58: recipe for target 'check' failed
[01:10:22] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1761d940
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 19 15:46:22 UTC 2018

plain
travis_time:end:055b1e6a:start=1559355632724391044,finish=1559355721082317976,duration=88357926932
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
[01:05:30] 
[01:05:30] running 143 tests
[01:05:32] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:05:34] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:34] 
[01:05:34]  finished in 4.630
[01:05:34] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:36] 
[01:05:36] running 9 tests
[01:05:36] iiiiiiiii
[01:05:36] 
[01:05:36]  finished in 0.150
[01:05:36] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:52] 
[01:05:52] running 122 tests
[01:06:18] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:23] .i.i......iii.i.....ii
[01:06:23] 
[01:06:23]  finished in 30.638
[01:06:23] travis_fold:end:test_debuginfo

---
[01:21:01]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:21:12] error[E0658]: use of unstable library feature 'bound_cloned'
[01:21:12]   --> src/libcore/../libcore/tests/ops.rs:88:41
[01:21:12]    |
[01:21:12] 88 |     assert_eq!(Bound::<&u32>::Unbounded.cloned(), Bound::Unbounded);
[01:21:12]    |
[01:21:12]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:21:12]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:21:12]    = help: add #![feature(bound_cloned)] to the crate attributes to enable
[01:21:12] error[E0658]: use of unstable library feature 'bound_cloned'
[01:21:12]   --> src/libcore/../libcore/tests/ops.rs:93:36
[01:21:12]    |
[01:21:12]    |
[01:21:12] 93 |     assert_eq!(Bound::Included(&3).cloned(), Bound::Included(3));
[01:21:12]    |
[01:21:12]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:21:12]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:21:12]    = help: add #![feature(bound_cloned)] to the crate attributes to enable
[01:21:12] error[E0658]: use of unstable library feature 'bound_cloned'
[01:21:12]   --> src/libcore/../libcore/tests/ops.rs:98:36
[01:21:12]    |
[01:21:12]    |
[01:21:12] 98 |     assert_eq!(Bound::Excluded(&3).cloned(), Bound::Excluded(3));
[01:21:12]    |
[01:21:12]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:21:12]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:21:12]    = help: add #![feature(bound_cloned)] to the crate attributes to enable
[01:21:14] error: aborting due to 3 previous errors
[01:21:14] 
[01:21:14] For more information about this error, try `rustc --explain E0658`.
[01:21:14] error: Could not compile `core`.
[01:21:14] error: Could not compile `core`.
[01:21:14] 
[01:21:14] To learn more, run the command again with --verbose.
[01:21:14] 
[01:21:14] 
[01:21:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:21:14] 
[01:21:14] 
[01:21:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:14] Build completed unsuccessfully in 1:17:03
[01:21:14] Build completed unsuccessfully in 1:17:03
j/build/bootstrap/debug/incremental
146428 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib
135320 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
135316 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fcqu72vki1-tdqsf8-2t7iak7gjee0p
123648 ./src/llvm-project/llvm/test/CodeGen
121960 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
108532 ./src/llvm-project/lldb
104940 ./obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-libstd-re-export
---
70836 ./obj/build/x8658355
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:023d1e00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02c793b6
travis_time:start:02c793b6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00618d68
$ dmesg | grep -i kill

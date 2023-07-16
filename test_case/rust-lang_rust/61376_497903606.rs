plain
travis_time:end:20d88950:start=1559349316005313853,finish=1559349403340076399,duration=87334762546
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
[01:08:52] 
[01:08:52] running 143 tests
[01:08:55] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:08:57] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:08:57] 
[01:08:57]  finished in 4.895
[01:08:57] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:59] 
[01:08:59] running 9 tests
[01:08:59] iiiiiiiii
[01:08:59] 
[01:08:59]  finished in 0.166
[01:08:59] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:16] 
[01:09:16] running 122 tests
[01:09:43] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:09:48] .i.i......iii.i.....ii
[01:09:48] 
[01:09:48]  finished in 31.913
[01:09:48] travis_fold:end:test_debuginfo

---
[01:25:14]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:25:26] error[E0658]: use of unstable library feature 'bound_cloned'
[01:25:26]   --> src/libcore/../libcore/tests/ops.rs:88:41
[01:25:26]    |
[01:25:26] 88 |     assert_eq!(Bound::<&u32>::Unbounded.cloned(), Bound::Unbounded);
[01:25:26]    |
[01:25:26]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:25:26]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:25:26]    = help: add #![feature(bound_cloned)] to the crate attributes to enable
[01:25:26] error[E0658]: use of unstable library feature 'bound_cloned'
[01:25:26]   --> src/libcore/../libcore/tests/ops.rs:93:36
[01:25:26]    |
[01:25:26]    |
[01:25:26] 93 |     assert_eq!(Bound::Included(&3).cloned(), Bound::Included(3));
[01:25:26]    |
[01:25:26]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:25:26]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:25:26]    = help: add #![feature(bound_cloned)] to the crate attributes to enable
[01:25:26] error[E0658]: use of unstable library feature 'bound_cloned'
[01:25:26]   --> src/libcore/../libcore/tests/ops.rs:98:36
[01:25:26]    |
[01:25:26]    |
[01:25:26] 98 |     assert_eq!(Bound::Excluded(&3).cloned(), Bound::Excluded(3));
[01:25:26]    |
[01:25:26]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:25:26]    = note: for more information, see https://github.com/rust-lang/rust/issues/61356
[01:25:26]    = help: add #![feature(bound_cloned)] to the crate attributes to enable
[01:25:28] error: aborting due to 3 previous errors
[01:25:28] 
[01:25:28] For more information about this error, try `rustc --explain E0658`.
[01:25:28] error: Could not compile `core`.
[01:25:28] error: Could not compile `core`.
[01:25:28] 
[01:25:28] To learn more, run the command again with --verbose.
[01:25:28] 
[01:25:28] 
[01:25:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:25:28] 
[01:25:28] 
[01:25:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:28] Build completed unsuccessfully in 1:21:31
---
travis_time:end:12740dcd:start=1559354543525347297,finish=1559354543530554865,duration=5207568
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0043e360
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29bded98
travis_time:start:29bded98
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0319a1dd
$ dmesg | grep -i kill

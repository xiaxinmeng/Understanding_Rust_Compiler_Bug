plain
travis_time:end:0f1ad89a:start=1559325577027206038,finish=1559325666892915007,duration=89865708969
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
[01:05:05] 
[01:05:05] running 143 tests
[01:05:08] i..iii.....iii..iiii.....i......................i...i................i......i.........ii.i..i..i.ii. 100/143
[01:05:09] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:09] 
[01:05:09]  finished in 4.507
[01:05:09] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:11] 
[01:05:11] running 9 tests
[01:05:11] iiiiiiiii
[01:05:11] 
[01:05:11]  finished in 0.150
[01:05:11] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:27] 
[01:05:27] running 122 tests
[01:05:51] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:05:56] .i.i......iii.i.....ii
[01:05:56] 
[01:05:56]  finished in 29.202
[01:05:56] travis_fold:end:test_debuginfo

---
[01:20:24]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:20:26] error[E0433]: failed to resolve: use of undeclared type or module `Bound`
[01:20:26]   --> src/libcore/../libcore/tests/ops.rs:88:16
[01:20:26]    |
[01:20:26] 88 |     assert_eq!(Bound::<&u32>::Unbounded.cloned(), Bound::Unbounded);
[01:20:26]    |                ^^^^^ use of undeclared type or module `Bound`
[01:20:26] error[E0433]: failed to resolve: use of undeclared type or module `Bound`
[01:20:26]   --> src/libcore/../libcore/tests/ops.rs:88:51
[01:20:26]    |
[01:20:26]    |
[01:20:26] 88 |     assert_eq!(Bound::<&u32>::Unbounded.cloned(), Bound::Unbounded);
[01:20:26]    |                                                   ^^^^^ use of undeclared type or module `Bound`
[01:20:26] error[E0433]: failed to resolve: use of undeclared type or module `Bound`
[01:20:26]   --> src/libcore/../libcore/tests/ops.rs:93:16
[01:20:26]    |
[01:20:26]    |
[01:20:26] 93 |     assert_eq!(Bound::Included(&3).cloned(), Bound::Included(3));
[01:20:26]    |                ^^^^^ use of undeclared type or module `Bound`
[01:20:26] error[E0433]: failed to resolve: use of undeclared type or module `Bound`
[01:20:26]   --> src/libcore/../libcore/tests/ops.rs:93:46
[01:20:26]    |
[01:20:26]    |
[01:20:26] 93 |     assert_eq!(Bound::Included(&3).cloned(), Bound::Included(3));
[01:20:26]    |                                              ^^^^^ use of undeclared type or module `Bound`
[01:20:26] error[E0433]: failed to resolve: use of undeclared type or module `Bound`
[01:20:26]   --> src/libcore/../libcore/tests/ops.rs:98:16
[01:20:26]    |
[01:20:26]    |
[01:20:26] 98 |     assert_eq!(Bound::Excluded(&3).cloned(), Bound::Excluded(3));
[01:20:26]    |                ^^^^^ use of undeclared type or module `Bound`
[01:20:26] error[E0433]: failed to resolve: use of undeclared type or module `Bound`
[01:20:26]   --> src/libcore/../libcore/tests/ops.rs:98:46
[01:20:26]    |
[01:20:26]    |
[01:20:26] 98 |     assert_eq!(Bound::Excluded(&3).cloned(), Bound::Excluded(3));
[01:20:26]    |                                              ^^^^^ use of undeclared type or module `Bound`
[01:20:38] error: aborting due to 6 previous errors
[01:20:38] 
[01:20:38] For more information about this error, try `rustc --explain E0433`.
[01:20:38] error: Could not compile `core`.
[01:20:38] error: Could not compile `core`.
[01:20:38] 
[01:20:38] To learn more, run the command again with --verbose.
[01:20:38] 
[01:20:38] 
[01:20:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:20:38] 
[01:20:38] 
[01:20:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:38] Build completed unsuccessfully in 1:16:19
---
travis_time:end:038f7ef4:start=1559330515795711461,finish=1559330515801331841,duration=5620380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0733ac78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13597014
travis_time:start:13597014
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true

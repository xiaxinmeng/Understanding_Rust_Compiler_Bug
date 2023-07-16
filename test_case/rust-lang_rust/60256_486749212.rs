plain
travis_time:end:17acb560:start=1556204635745974475,finish=1556204637832352408,duration=2086377933
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:28] 
[01:20:28] running 9 tests
[01:20:28] iiiiiiiii
[01:20:28] 
[01:20:28]  finished in 0.146
[01:20:28] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:44] 
[01:20:44] running 121 tests
[01:21:09] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:21:14] i.i......iii.i.....ii
[01:21:14] 
[01:21:14]  finished in 30.498
[01:21:14] travis_fold:end:test_debuginfo

---
[01:36:27] .................................................................................................... 1600/2309
[01:36:39] .................................................................................................... 1700/2309
[01:36:53] .................................................................................................... 1800/2309
[01:37:07] .................................................................................................... 1900/2309
[01:37:20] .......................F............................................................................ 2000/2309
[01:37:53] .................................................................................................... 2200/2309
[01:38:10] ................i................................................................................... 2300/2309
[01:38:11] .........
[01:38:11] failures:
[01:38:11] failures:
[01:38:11] 
[01:38:11] ---- option.rs - option::Option<Option<T>>::flatten (line 1424) stdout ----
[01:38:11] error[E0658]: use of unstable library feature 'option_flattening'
[01:38:11]  --> option.rs:1426:23
[01:38:11]   |
[01:38:11] 5 | assert_eq!(Some(6), x.flatten());
[01:38:11]   |
[01:38:11]   = note: for more information, see https://github.com/rust-lang/rust/issues/60258
[01:38:11]   = note: for more information, see https://github.com/rust-lang/rust/issues/60258
[01:38:11]   = help: add #![feature(option_flattening)] to the crate attributes to enable
[01:38:11] error[E0658]: use of unstable library feature 'option_flattening'
[01:38:11]  --> option.rs:1429:20
[01:38:11]   |
[01:38:11]   |
[01:38:11] 8 | assert_eq!(None, x.flatten());
[01:38:11]   |
[01:38:11]   = note: for more information, see https://github.com/rust-lang/rust/issues/60258
[01:38:11]   = note: for more information, see https://github.com/rust-lang/rust/issues/60258
[01:38:11]   = help: add #![feature(option_flattening)] to the crate attributes to enable
[01:38:11] error[E0658]: use of unstable library feature 'option_flattening'
[01:38:11]   --> option.rs:1432:20
[01:38:11]    |
[01:38:11]    |
[01:38:11] 11 | assert_eq!(None, x.flatten());
[01:38:11]    |
[01:38:11]    = note: for more information, see https://github.com/rust-lang/rust/issues/60258
[01:38:11]    = note: for more information, see https://github.com/rust-lang/rust/issues/60258
[01:38:11]    = help: add #![feature(option_flattening)] to the crate attributes to enable
[01:38:11] error: aborting due to 3 previous errors
[01:38:11] 
[01:38:11] For more information about this error, try `rustc --explain E0658`.
[01:38:11] For more information about this error, try `rustc --explain E0658`.
[01:38:11] thread 'option.rs - option::Option<Option<T>>::flatten (line 1424)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:38:11] 
[01:38:11] 
[01:38:11] failures:
[01:38:11]     option.rs - option::Option<Option<T>>::flatten (line 1424)
[01:38:11]     option.rs - option::Option<Option<T>>::flatten (line 1424)
[01:38:11] 
[01:38:11] test result: FAILED. 2297 passed; 1 failed; 11 ignored; 0 measured; 0 filtered out
[01:38:11] 
[01:38:11] error: test failed, to rerun pass '--doc'
[01:38:11] 
[01:38:11] 
[01:38:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:38:11] 
[01:38:11] 
[01:38:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:11] Build completed unsuccessfully in 0:29:16
[01:38:11] Build completed unsuccessfully in 0:29:16
[01:38:11] make: *** [check] Error 1
[01:38:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0028eda4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 16:42:20 UTC 2019
---
travis_time:end:0857dbe6:start=1556210542191347728,finish=1556210542245854899,duration=54507171
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:059e1a58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:079e6d16
$ dmesg | grep -i kill

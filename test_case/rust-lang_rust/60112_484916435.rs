plain
travis_time:end:04a321ac:start=1555679515669706898,finish=1555679609158484179,duration=93488777281
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
[01:15:40] 
[01:15:40] running 9 tests
[01:15:40] iiiiiiiii
[01:15:40] 
[01:15:40]  finished in 0.156
[01:15:40] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:57] 
[01:15:57] running 121 tests
[01:16:23] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:16:28] i.i......iii.i.....ii
[01:16:28] 
[01:16:28]  finished in 31.094
[01:16:28] travis_fold:end:test_debuginfo

---
[01:26:47]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:26:52] error[E0277]: `std::string::String` is not an iterator
[01:26:52]   --> src/libcore/../libcore/tests/char.rs:85:36
[01:26:52]    |
[01:26:52] 85 |         assert!(iter.chars().rev().eq(rev_iter));
[01:26:52]    |                                    ^^ `std::string::String` is not an iterator; try calling `.chars()` or `.bytes()`
[01:26:52]    = help: the trait `std::iter::Iterator` is not implemented for `std::string::String`
[01:26:52]    = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `std::string::String`
[01:26:52] 
[01:26:52] error[E0277]: `std::string::String` is not an iterator
[01:26:52] error[E0277]: `std::string::String` is not an iterator
[01:26:52]    --> src/libcore/../libcore/tests/char.rs:114:36
[01:26:52]     |
[01:26:52] 114 |         assert!(iter.chars().rev().eq(rev_iter));
[01:26:52]     |                                    ^^ `std::string::String` is not an iterator; try calling `.chars()` or `.bytes()`
[01:26:52]     = help: the trait `std::iter::Iterator` is not implemented for `std::string::String`
[01:26:52]     = note: required because of the requirements on the impl of `std::iter::IntoIterator` for `std::string::String`
[01:26:52] 
[01:27:01] error: aborting due to 2 previous errors
[01:27:01] error: aborting due to 2 previous errors
[01:27:01] 
[01:27:01] For more information about this error, try `rustc --explain E0277`.
[01:27:01] error: Could not compile `core`.
[01:27:01] 
[01:27:01] To learn more, run the command again with --verbose.
[01:27:01] 
[01:27:01] 
[01:27:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:27:01] 
[01:27:01] 
[01:27:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:01] Build completed unsuccessfully in 0:23:25
[01:27:01] Build completed unsuccessfully in 0:23:25
[01:27:01] make: *** [check] Error 1
[01:27:01] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0efa8fa0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 19 14:40:40 UTC 2019
---
travis_time:end:250152c3:start=1555684842286782040,finish=1555684842361534126,duration=74752086
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:073ddaf0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3c707b55
$ dmesg | grep -i kill

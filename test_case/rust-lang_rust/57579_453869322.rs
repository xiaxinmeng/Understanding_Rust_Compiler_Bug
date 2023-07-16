plain
travis_time:end:1a5c3880:start=1547411138559837760,finish=1547411227197742201,duration=88637904441
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
[01:16:40] 
[01:16:40] running 118 tests
[01:17:07] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:17:12] ......iii.i.....ii
[01:17:12] 
[01:17:12]  finished in 31.861
[01:17:12] travis_fold:end:test_debuginfo

---
[01:29:23] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:29:24]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:29:43] error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
[01:29:43]      |
[01:29:43]      |
[01:29:43] 1912 |     let mut it = once_with(|| {
[01:29:43]      |                            -- mutable borrow occurs here
[01:29:43] 1913 |         count += 1;
[01:29:43]      |         ----- previous borrow occurs due to use of `count` in closure
[01:29:43] 1917 |     assert_eq!(count, 0);
[01:29:43]      |                ^^^^^ immutable borrow occurs here
[01:29:43] ...
[01:29:43] 1924 | }
[01:29:43] 1924 | }
[01:29:43]      | - mutable borrow ends here
[01:29:43] 
[01:29:43] error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
[01:29:43]      |
[01:29:43]      |
[01:29:43] 1912 |     let mut it = once_with(|| {
[01:29:43]      |                            -- mutable borrow occurs here
[01:29:43] 1913 |         count += 1;
[01:29:43]      |         ----- previous borrow occurs due to use of `count` in closure
[01:29:43] 1919 |     assert_eq!(count, 1);
[01:29:43]      |                ^^^^^ immutable borrow occurs here
[01:29:43] ...
[01:29:43] 1924 | }
[01:29:43] 1924 | }
[01:29:43]      | - mutable borrow ends here
[01:29:43] 
[01:29:43] error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
[01:29:43]      |
[01:29:43]      |
[01:29:43] 1912 |     let mut it = once_with(|| {
[01:29:43]      |                            -- mutable borrow occurs here
[01:29:43] 1913 |         count += 1;
[01:29:43]      |         ----- previous borrow occurs due to use of `count` in closure
[01:29:43] 1921 |     assert_eq!(count, 1);
[01:29:43]      |                ^^^^^ immutable borrow occurs here
[01:29:43] ...
[01:29:43] 1924 | }
[01:29:43] 1924 | }
[01:29:43]      | - mutable borrow ends here
[01:29:43] 
[01:29:43] error[E0502]: cannot borrow `count` as immutable because it is also borrowed as mutable
[01:29:43]      |
[01:29:43]      |
[01:29:43] 1912 |     let mut it = once_with(|| {
[01:29:43]      |                            -- mutable borrow occurs here
[01:29:43] 1913 |         count += 1;
[01:29:43]      |         ----- previous borrow occurs due to use of `count` in closure
[01:29:43] 1923 |     assert_eq!(count, 1);
[01:29:43]      |                ^^^^^ immutable borrow occurs here
[01:29:43] 1924 | }
[01:29:43] 1924 | }
[01:29:43]      | - mutable borrow ends here
[01:29:46] error: aborting due to 4 previous errors
[01:29:46] 
[01:29:46] For more information about this error, try `rustc --explain E0502`.
[01:29:46] error: Could not compile `core`.
[01:29:46] error: Could not compile `core`.
[01:29:46] 
[01:29:46] To learn more, run the command again with --verbose.
[01:29:46] 
[01:29:46] 
[01:29:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:29:46] 
[01:29:46] 
[01:29:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:46] Build completed unsuccessfully in 0:26:00
[01:29:46] Build completed unsuccessfully in 0:26:00
[01:29:46] make: *** [check] Error 1
[01:29:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09ba4826
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 21:57:03 UTC 2019
---
travis_time:end:28e27d61:start=1547416625403579804,finish=1547416625462675408,duration=59095604
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:067442e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01c81334
$ dmesg | grep -i kill

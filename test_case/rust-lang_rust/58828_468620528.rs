plain
travis_time:end:2809a213:start=1551430843420137168,finish=1551430844431612811,duration=1011475643
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
[01:15:59] 
[01:15:59] running 119 tests
[01:16:24] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:16:28] i......iii.i.....ii
[01:16:28] 
[01:16:28]  finished in 29.335
[01:16:28] travis_fold:end:test_debuginfo

---
[01:32:15] 
[01:32:15] error: hidden lifetime parameters in types are deprecated
[01:32:15]    --> src/libstd/io/stdio.rs:768:30
[01:32:15]     |
[01:32:15] 768 |         assert_unwind_safe::<StdoutLock>();
[01:32:15] 
[01:32:15] error: hidden lifetime parameters in types are deprecated
[01:32:15]    --> src/libstd/io/stdio.rs:777:30
[01:32:15]     |
[01:32:15]     |
[01:32:15] 777 |         assert_unwind_safe::<StderrLock>();
[01:32:15] 
[01:32:15] error: hidden lifetime parameters in types are deprecated
[01:32:15]     --> src/libstd/path.rs:2918:36
[01:32:15]      |
[01:32:15]      |
[01:32:15] 2918 |             let borrowed_cow_path: Cow<Path> = path.into();
[01:32:15]      |                                    ^^^^^^^^^ help: indicate the anonymous lifetime: `Cow<'_, Path>`
[01:32:15] error: hidden lifetime parameters in types are deprecated
[01:32:15]     --> src/libstd/path.rs:4016:27
[01:32:15]      |
[01:32:15]      |
[01:32:15] 4016 |         let borrowed_cow: Cow<Path> = borrowed.into();
[01:32:15]      |                           ^^^^^^^^^ help: indicate the anonymous lifetime: `Cow<'_, Path>`
[01:32:15] error: hidden lifetime parameters in types are deprecated
[01:32:15]     --> src/libstd/path.rs:4017:24
[01:32:15]      |
[01:32:15]      |
[01:32:15] 4017 |         let owned_cow: Cow<Path> = owned.clone().into();
[01:32:15]      |                        ^^^^^^^^^ help: indicate the anonymous lifetime: `Cow<'_, Path>`
[01:32:15] error: hidden lifetime parameters in types are deprecated
[01:32:15]     --> src/libstd/sys_common/wtf8.rs:1231:23
[01:32:15]      |
[01:32:15]      |
[01:32:15] 1231 |         let expected: Cow<str> = Cow::Owned(String::from("aé 💩�"));
[01:32:15] 
[01:32:25] error: aborting due to 7 previous errors
[01:32:25] 
[01:32:25] error: Could not compile `std`.
[01:32:25] error: Could not compile `std`.
[01:32:25] 
[01:32:25] To learn more, run the command again with --verbose.
[01:32:25] 
[01:32:25] 
[01:32:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:32:25] 
[01:32:25] 
[01:32:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:25] Build completed unsuccessfully in 0:28:12
[01:32:25] Build completed unsuccessfully in 0:28:12
[01:32:25] make: *** [check] Error 1
[01:32:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07895a5a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 10:33:20 UTC 2019

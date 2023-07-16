plain
travis_time:end:0e2275b9:start=1550385656777441921,finish=1550385659121397795,duration=2343955874
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
[01:15:23] 
[01:15:23] running 119 tests
[01:15:48] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:15:53] i......iii.i.....ii
[01:15:53] 
[01:15:53]  finished in 29.926
[01:15:53] travis_fold:end:test_debuginfo

---
[01:37:52] 130 | |             &sess,
[01:37:52] 131 | |             &cstore,
[01:37:52] 132 | |             krate,
[01:37:52] ...   |
[01:37:52] 136 | |             |_| Ok(()),
[01:37:52] 137 | |         ).expect("phase 2 aborted")
[01:37:52]     | 
[01:37:52]    ::: src/librustc_driver/driver.rs:734:1
[01:37:52]     |
[01:37:52]     |
[01:37:52] 734 | / pub fn phase_2_configure_and_expand<'a, F>(
[01:37:52] 735 | |     sess: &'a Session,
[01:37:52] 736 | |     cstore: &'a CStore,
[01:37:52] 737 | |     krate: ast::Crate,
[01:37:52] 790 | |     }
[01:37:52] 791 | | }
[01:37:52]     | |_- defined here
[01:37:52] 
---
[01:37:53] 
[01:37:53] To learn more, run the command again with --verbose.
[01:37:53] 
[01:37:53] 
[01:37:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:37:53] 
[01:37:53] 
[01:37:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:37:53] Build completed unsuccessfully in 0:34:19
[01:37:53] Build completed unsuccessfully in 0:34:19
[01:37:53] make: *** [check] Error 1
[01:37:53] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2958e3b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 08:19:06 UTC 2019
---
travis_time:end:25ceb3e1:start=1550391548453791537,finish=1550391548521109927,duration=67318390
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0672fab9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d84c9ef
$ dmesg | grep -i kill

plain
travis_time:end:20db9100:start=1549465571116619232,finish=1549465573851959489,duration=2735340257
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
[01:09:01] 
[01:09:01] running 119 tests
[01:09:29] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:34] i......iii.i.....ii
[01:09:34] 
[01:09:34]  finished in 33.026
[01:09:34] travis_fold:end:test_debuginfo

---
[01:33:02]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:33:05] error: hidden lifetime parameters in types are deprecated
[01:33:05]     --> src/libsyntax/mut_visit.rs:1268:33
[01:33:05]      |
[01:33:05] 1268 |     fn fake_print_crate(s: &mut pprust::State,
[01:33:05] 
[01:33:12] error: aborting due to previous error
[01:33:12] 
[01:33:12] error: Could not compile `syntax`.
[01:33:12] error: Could not compile `syntax`.
[01:33:12] 
[01:33:12] To learn more, run the command again with --verbose.
[01:33:12] 
[01:33:12] 
[01:33:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:33:12] 
[01:33:12] 
[01:33:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:12] Build completed unsuccessfully in 0:36:20
[01:33:12] Build completed unsuccessfully in 0:36:20
[01:33:12] make: *** [check] Error 1
[01:33:12] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0629b592
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 16:39:37 UTC 2019

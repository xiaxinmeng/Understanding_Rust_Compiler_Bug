plain
travis_time:end:05e485c0:start=1555199546374667017,finish=1555199547111449582,duration=736782565
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
[01:13:41] 
[01:13:41] running 9 tests
[01:13:41] iiiiiiiii
[01:13:41] 
[01:13:41]  finished in 0.154
[01:13:41] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:58] 
[01:13:58] running 121 tests
[01:14:23] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:28] i.i......iii.i.....ii
[01:14:28] 
[01:14:28]  finished in 30.513
[01:14:28] travis_fold:end:test_debuginfo

---
travis_time:start:test_stage2-test
Testing test stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:56]    Compiling getopts v0.2.17
[01:21:59]    Compiling test v0.0.0 (/checkout/src/libtest)
[01:21:59] error[E0523]: found two different crates with name `getopts` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[01:21:59]    |
[01:21:59] 34 | use getopts;
[01:21:59]    |     ^^^^^^^
[01:21:59] 
---
[01:21:59] warning: build failed, waiting for other jobs to finish...
[01:22:09] error: build failed
[01:22:09] 
[01:22:09] 
[01:22:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test" "--" "--quiet"
[01:22:09] 
[01:22:09] 
[01:22:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:09] Build completed unsuccessfully in 0:20:12
[01:22:09] Build completed unsuccessfully in 0:20:12
[01:22:09] Makefile:48: recipe for target 'check' failed
[01:22:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06ef1ad4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 14 01:14:47 UTC 2019

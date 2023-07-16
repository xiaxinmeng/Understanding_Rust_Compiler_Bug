plain
travis_time:end:223b8c58:start=1552815491819488576,finish=1552815492721963070,duration=902474494
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:49] 
[01:22:49] running 120 tests
[01:23:15] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:23:20] .i......iii.i.....ii
[01:23:20] 
[01:23:20]  finished in 31.543
[01:23:20] travis_fold:end:test_debuginfo

---
[01:46:11] 
[01:46:11] running 9 tests
[01:46:11] thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 4294967040', /checkout/src/libcore/slice/mod.rs:2539:10
[01:46:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:46:11] ..F......
[01:46:11] 
[01:46:11] failures:
[01:46:11]     test::contravariant_region_ptr_err
[01:46:11] 
[01:46:11] 
[01:46:11] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:46:11] 
[01:46:11] error: test failed, to rerun pass '--lib'
[01:46:11] 
[01:46:11] 
[01:46:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:46:11] 
[01:46:11] 
[01:46:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:11] Build completed unsuccessfully in 0:35:46
[01:46:11] Build completed unsuccessfully in 0:35:46
[01:46:11] make: *** [check] Error 1
[01:46:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0349b170
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 17 11:24:34 UTC 2019

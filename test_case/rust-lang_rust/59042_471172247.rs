plain
travis_time:end:111a9abc:start=1552127067054046443,finish=1552127142662952951,duration=75608906508
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
[01:27:42] 
[01:27:42] running 119 tests
[01:28:10] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:28:14] i......iii.i.....ii
[01:28:14] 
[01:28:14]  finished in 32.648
[01:28:14] travis_fold:end:test_debuginfo

---
[01:52:03]     Finished release [optimized] target(s) in 47.88s
[01:52:03]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_driver-72f48af8359eb542
[01:52:03] 
[01:52:03] running 9 tests
[01:52:03] ...F.....
[01:52:03] 
[01:52:03] ---- test::contravariant_region_ptr_err stdout ----
[01:52:03] ---- test::contravariant_region_ptr_err stdout ----
[01:52:03] thread 'test::contravariant_region_ptr_err' panicked at 'index out of bounds: the len is 1 but the index is 4294967040', /checkout/src/libcore/slice/mod.rs:2539:10
[01:52:03] 
[01:52:03] 
[01:52:03] failures:
[01:52:03]     test::contravariant_region_ptr_err
[01:52:03]     test::contravariant_region_ptr_err
[01:52:03] 
[01:52:03] test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:52:03] 
[01:52:03] error: test failed, to rerun pass '--lib'
[01:52:03] 
[01:52:03] 
[01:52:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:52:03] 
[01:52:03] 
[01:52:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:52:03] Build completed unsuccessfully in 0:37:41
[01:52:03] Build completed unsuccessfully in 0:37:41
[01:52:03] make: *** [check] Error 1
[01:52:03] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bfdefd4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  9 12:17:55 UTC 2019
---
travis_time:end:06567592:start=1552133877784669710,finish=1552133877790069673,duration=5399963
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:020e05ea
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=

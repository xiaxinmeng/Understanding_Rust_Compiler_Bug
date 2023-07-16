plain
travis_time:end:0329a06c:start=1551977265745313705,finish=1551977266643922442,duration=898608737
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
[01:24:46] 
[01:24:46] running 119 tests
[01:25:12] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:25:17] i......iii.i.....ii
[01:25:17] 
[01:25:17]  finished in 30.307
[01:25:17] travis_fold:end:test_debuginfo

---
[01:47:38]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:47:39] error[E0308]: mismatched types
[01:47:39]    --> src/librustc_driver/test.rs:116:66
[01:47:39]     |
[01:47:39] 116 |     let diagnostic_handler = errors::Handler::with_emitter(true, false, emitter);
[01:47:39]     |
[01:47:39]     = note: expected type `std::option::Option<usize>`
[01:47:39]                found type `bool`
[01:47:39] 
---
[01:47:40] 
[01:47:40] To learn more, run the command again with --verbose.
[01:47:40] 
[01:47:40] 
[01:47:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:47:40] 
[01:47:40] 
[01:47:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:47:40] Build completed unsuccessfully in 0:35:05
[01:47:40] Build completed unsuccessfully in 0:35:05
[01:47:40] make: *** [check] Error 1
[01:47:40] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:158e3dc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar  7 18:35:38 UTC 2019

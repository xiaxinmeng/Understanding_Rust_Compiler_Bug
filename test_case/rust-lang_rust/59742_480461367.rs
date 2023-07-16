plain
travis_time:end:08416134:start=1554506652255525306,finish=1554506653235880580,duration=980355274
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:51] 
[01:13:51] running 9 tests
[01:13:51] iiiiiiiii
[01:13:51] 
[01:13:51]  finished in 0.152
[01:13:51] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:07] 
[01:14:07] running 121 tests
[01:14:33] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:38] i.i......iii.i.....ii
[01:14:38] 
[01:14:38]  finished in 31.042
[01:14:38] travis_fold:end:test_debuginfo

---
[01:38:53] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:53]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:39:03] error[E0063]: missing field `edition` in initializer of `parse::ParseSess`
[01:39:03]      |
[01:39:03] 1924 |         ParseSess {
[01:39:03]      |         ^^^^^^^^^ missing `edition`
[01:39:03] 
---
[01:39:05] 
[01:39:05] To learn more, run the command again with --verbose.
[01:39:05] 
[01:39:05] 
[01:39:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:39:05] 
[01:39:05] 
[01:39:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:05] Build completed unsuccessfully in 0:36:59
[01:39:05] Build completed unsuccessfully in 0:36:59
[01:39:05] make: *** [check] Error 1
[01:39:05] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:017a6024
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 01:03:30 UTC 2019

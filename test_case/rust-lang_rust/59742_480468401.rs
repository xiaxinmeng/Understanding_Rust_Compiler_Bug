plain
travis_time:end:20d59e2c:start=1554513197575616414,finish=1554513199746810960,duration=2171194546
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
[01:17:14] 
[01:17:14] running 9 tests
[01:17:14] iiiiiiiii
[01:17:14] 
[01:17:14]  finished in 0.162
[01:17:14] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:31] 
[01:17:31] running 121 tests
[01:17:57] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:02] i.i......iii.i.....ii
[01:18:02] 
[01:18:02]  finished in 31.373
[01:18:02] travis_fold:end:test_debuginfo

---
[01:44:35]   |
[01:44:35] 1 | ::abc::def::r#return
[01:44:35]   |             ^^^^^^^^
[01:44:35] 
[01:44:35] ......................................................................F.........
[01:44:35] 
[01:44:35] ---- tokenstream::tests::test_dotdotdot stdout ----
[01:44:35] ---- tokenstream::tests::test_dotdotdot stdout ----
[01:44:35] thread 'tokenstream::tests::test_dotdotdot' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
[01:44:35] 
[01:44:35] 
[01:44:35] failures:
[01:44:35]     tokenstream::tests::test_dotdotdot
[01:44:35]     tokenstream::tests::test_dotdotdot
[01:44:35] 
[01:44:35] test result: FAILED. 79 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:35] 
[01:44:35] error: test failed, to rerun pass '--lib'
[01:44:35] 
[01:44:35] 
[01:44:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:44:35] 
[01:44:35] 
[01:44:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:35] Build completed unsuccessfully in 0:39:31
[01:44:35] Build completed unsuccessfully in 0:39:31
[01:44:35] make: *** [check] Error 1
[01:44:35] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0346e0b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr  6 02:58:05 UTC 2019

plain
travis_time:end:08669ad0:start=1553520263098209951,finish=1553520385044480220,duration=121946270269
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
[01:22:50] 
[01:22:50] running 9 tests
[01:22:50] iiiiiiiii
[01:22:50] 
[01:22:50]  finished in 0.155
[01:22:50] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:06] 
[01:23:06] running 120 tests
[01:23:32] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:23:36] .i......iii.i.....ii
[01:23:36] 
[01:23:36]  finished in 30.460
[01:23:36] travis_fold:end:test_debuginfo

---
[01:44:00] travis_fold:start:test_stage1-rustc
travis_time:start:test_stage1-rustc
Testing rustc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:44:01]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:44:07] error[E0433]: failed to resolve: use of undeclared type or module `HumanReadableErrorType`
[01:44:07]     --> src/librustc/session/config.rs:2913:28
[01:44:07]      |
[01:44:07] 2913 |             json_rendered: HumanReadableErrorType::Default(ColorConfig::Never),
[01:44:07]      |                            ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `HumanReadableErrorType`
[01:44:07] error[E0433]: failed to resolve: use of undeclared type or module `ColorConfig`
[01:44:07]     --> src/librustc/session/config.rs:2913:60
[01:44:07]      |
[01:44:07]      |
[01:44:07] 2913 |             json_rendered: HumanReadableErrorType::Default(ColorConfig::Never),
[01:44:07] 
[01:44:35] error: aborting due to 2 previous errors
[01:44:35] 
[01:44:35] For more information about this error, try `rustc --explain E0433`.
[01:44:35] For more information about this error, try `rustc --explain E0433`.
[01:44:36] error: Could not compile `rustc`.
[01:44:36] 
[01:44:36] To learn more, run the command again with --verbose.
[01:44:36] 
[01:44:36] 
[01:44:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:44:36] 
[01:44:36] 
[01:44:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:36] Build completed unsuccessfully in 0:33:30
[01:44:36] Build completed unsuccessfully in 0:33:30
[01:44:36] Makefile:48: recipe for target 'check' failed
[01:44:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ca6ce14
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 25 15:11:09 UTC 2019

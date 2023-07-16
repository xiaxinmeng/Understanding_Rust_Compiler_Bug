plain
travis_time:end:05de7600:start=1555638070328596215,finish=1555638071252962716,duration=924366501
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
[01:17:22] 
[01:17:22] running 9 tests
[01:17:22] iiiiiiiii
[01:17:22] 
[01:17:22]  finished in 0.161
[01:17:22] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:38] 
[01:17:38] running 121 tests
[01:18:05] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:09] i.i......iii.i.....ii
[01:18:09] 
[01:18:09]  finished in 31.250
[01:18:09] travis_fold:end:test_debuginfo

---
[01:44:09] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:44:09]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:44:20] error[E0063]: missing field `missing_ident_could_be_struct_literal` in initializer of `parse::ParseSess`
[01:44:20]      |
[01:44:20] 1926 |         ParseSess {
[01:44:20] 1926 |         ParseSess {
[01:44:20]      |         ^^^^^^^^^ missing `missing_ident_could_be_struct_literal`
[01:44:22] error: aborting due to previous error
[01:44:22] 
[01:44:22] For more information about this error, try `rustc --explain E0063`.
[01:44:22] error: Could not compile `syntax`.
[01:44:22] error: Could not compile `syntax`.
[01:44:22] 
[01:44:22] To learn more, run the command again with --verbose.
[01:44:22] 
[01:44:22] 
[01:44:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:44:22] 
[01:44:22] 
[01:44:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:22] Build completed unsuccessfully in 0:39:10
[01:44:22] Build completed unsuccessfully in 0:39:10
[01:44:22] Makefile:48: recipe for target 'check' failed
[01:44:22] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09200860
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 19 03:25:44 UTC 2019

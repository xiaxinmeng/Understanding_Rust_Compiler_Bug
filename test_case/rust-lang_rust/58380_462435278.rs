plain
travis_time:end:17e47445:start=1549908053574447292,finish=1549908054477051944,duration=902604652
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:13:06]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:08] error: hidden lifetime parameters in types are deprecated
[00:13:08]    --> src/librustc_mir/hair/pattern/check_match.rs:487:82
[00:13:08]     |
[00:13:08] 487 |             let patterns = witnesses.iter().map(|p| (**p).clone()).collect::<Vec<Pattern>>();
[00:13:08] 
[00:13:08] error: hidden lifetime parameters in types are deprecated
[00:13:08]    --> src/librustc_mir/hair/pattern/check_match.rs:513:22
[00:13:08]     |
[00:13:08]     |
[00:13:08] 513 |             let pk: &PatternKind = &pattern.kind;
[00:13:08] 
[00:13:08] error: hidden lifetime parameters in types are deprecated
[00:13:08]    --> src/librustc_mir/hair/pattern/check_match.rs:507:17
[00:13:08]     |
[00:13:08]     |
[00:13:08] 507 |     patterns: &[Pattern],
[00:13:08] 
[00:13:20] error: aborting due to 3 previous errors
[00:13:20] 
[00:13:20] error: Could not compile `rustc_mir`.
[00:13:20] error: Could not compile `rustc_mir`.
[00:13:20] warning: build failed, waiting for other jobs to finish...
[00:16:09] error: build failed
[00:16:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:09] expected success, got: exit code: 101
[00:16:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:09] Build completed unsuccessfully in 0:11:43
[00:16:09] make: *** [all] Error 1
[00:16:09] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04c8f36c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 18:17:15 UTC 2019
---
travis_time:end:078a79b8:start=1549909035604279492,finish=1549909036162123460,duration=557843968
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:204d2abc
$ ls -lat $HOME/Library/Logs/DiagnosticRean/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:368e707d
$ dmesg | grep -i kill

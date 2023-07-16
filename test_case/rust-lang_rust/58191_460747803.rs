plain
travis_time:end:1204b4f8:start=1549389709507030940,finish=1549389711806624270,duration=2299593330
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:21:02]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:21:04] error: unused variable: `ty`
[00:21:04]    --> src/librustc_save_analysis/sig.rs:657:56
[00:21:04]     |
[00:21:04] 657 |                     ast::GenericParamKind::Const { ref ty } => {
[00:21:04]     |                                                        ^^ help: try ignoring the field: `ty: _`
[00:21:04]     = note: `-D unused-variables` implied by `-D warnings`
[00:21:04] 
[00:21:04] error: aborting due to previous error
[00:21:04] 
[00:21:04] 
[00:21:04] error: Could not compile `rustc_save_analysis`.
[00:21:04] warning: build failed, waiting for other jobs to finish...
[00:21:24] error: build failed
[00:21:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:21:24] expected success, got: exit code: 101
[00:21:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:24] Build completed unsuccessfully in 0:17:13
[00:21:24] make: *** [all] Error 1
[00:21:24] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06762885
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 18:23:26 UTC 2019
---
travis_time:end:1b7c164f:start=1549391007387783751,finish=1549391007392724859,duration=4941108
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07cf7e10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19e2c38e
travis_time:start:19e2c38e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23ea4be2
$ dmesg | grep -i kill

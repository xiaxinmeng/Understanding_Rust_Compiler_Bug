plain
travis_time:end:00d4a60c:start=1544289288835307124,finish=1544289377476586589,duration=88641279465
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:12:57]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:58] error[E0603]: trait `QueryConfig` is private
[00:12:58]   --> src/librustc_metadata/cstore_impl.rs:18:23
[00:12:58]    |
[00:12:58] 18 | use rustc::ty::query::QueryConfig;
[00:12:58] 
[00:12:59] error: aborting due to previous error
[00:12:59] 
[00:12:59] For more information about this error, try `rustc --explain E0603`.
[00:12:59] For more information about this error, try `rustc --explain E0603`.
[00:12:59] error: Could not compile `rustc_metadata`.
[00:12:59] warning: build failed, waiting for other jobs to finish...
[00:16:19] error: build failed
[00:16:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:19] expected success, got: exit code: 101
[00:16:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:19] Build completed unsuccessfully in 0:12:58
[00:16:19] make: *** [all] Error 1
[00:16:19] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1192bcea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 17:32:45 UTC 2018
---
travis_time:end:1f624874:start=1544290366254776286,finish=1544290366259613716,duration=4837430
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0eacacbd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d987250
travis_time:start:1d987250
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13ff22af
$ dmesg | grep -i kill

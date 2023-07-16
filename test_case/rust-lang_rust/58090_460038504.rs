plain
travis_time:end:0d6ef74a:start=1549186175652682521,finish=1549186176522873573,duration=870191052
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:24:50] 
[00:24:50] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:24:50] 
[00:24:50] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:24:50] note: some of the compiler flags provided by cargo are hidden
[00:24:50] 
[00:24:50] error: Could not compile `core`.
[00:24:50] warning: build failed, waiting for other jobs to finish...
[00:24:50] warning: build failed, waiting for other jobs to finish...
[00:24:55] error: build failed
[00:24:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:24:55] expected success, got: exit code: 101
[00:24:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:55] Build completed unsuccessfully in 0:20:43
[00:24:55] make: *** [all] Error 1
[00:24:55] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:071d8872
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 09:54:43 UTC 2019
---
travis_time:end:126d2ae0:start=1549187683763573155,finish=1549187683768337482,duration=4764327
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05a57740
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0338c1a8
travis_time:start:0338c1a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:035ffb1d
$ dmesg | grep -i kill

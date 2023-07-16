plain
travis_time:end:07dc27de:start=1547826919829939815,finish=1547826923299526689,duration=3469586874
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:26:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:25] 
[00:26:25] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[00:26:25] 
[00:26:25] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:26:25] note: some of the compiler flags provided by cargo are hidden
[00:26:25] 
[00:26:25] error: Could not compile `core`.
[00:26:25] warning: build failed, waiting for other jobs to finish...
[00:26:25] warning: build failed, waiting for other jobs to finish...
[00:26:26] error: build failed
[00:26:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:26:26] expected success, got: exit code: 101
[00:26:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:26] Build completed unsuccessfully in 0:22:23
[00:26:26] Makefile:18: recipe for target 'all' failed
[00:26:26] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e0be6fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 18 16:22:00 UTC 2019
---
travis_time:end:131ad4c1:start=1547828520873775149,finish=1547828520878493945,duration=4718796
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bdaf416
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ea00828
travis_time:start:0ea00828
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c2950c4
$ dmesg | grep -i kill

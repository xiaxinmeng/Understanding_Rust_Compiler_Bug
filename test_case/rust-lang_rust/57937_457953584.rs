plain
travis_time:end:2f5eff36:start=1548620577801138403,finish=1548620580132025617,duration=2330887214
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:30] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:30] tidy error: /checkout/src/librustc_target/spec/nvptx64_nvidia_cuda.rs:27: TODO is deprecated; use FIXME
[00:03:30] tidy error: /checkout/src/librustc_target/spec/nvptx64_nvidia_cuda.rs:54: TODO is deprecated; use FIXME
[00:03:32] some tidy checks failed
[00:03:32] 
[00:03:32] 
[00:03:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:32] 
[00:03:32] 
[00:03:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:32] Build completed unsuccessfully in 0:00:48
[00:03:32] Build completed unsuccessfully in 0:00:48
[00:03:32] Makefile:68: recipe for target 'tidy' failed
[00:03:32] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0684e740
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 27 20:26:44 UTC 2019
---
travis_time:end:1612cfd0:start=1548620805407301115,finish=1548620805412457762,duration=5156647
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02355b0b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0524b050
travis_time:start:0524b050
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21722ef8
$ dmesg | grep -i kill

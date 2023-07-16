plain
travis_fold:start:worker_info
Worker information
hostname: c2281654-c580-437f-b94d-53fd6b51bf71@1.production-1-worker-com-gce-dhzm
version: v6.2.0 https://github.com/travis-ci/worker/tree/5e5476e01646095f48eec13196fdb3faf8f5cbf7
startup: 6.537871547s
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
---
travis_time:end:08a5bddf:start=1547152926307852151,finish=1547152927417908077,duration=1110055926
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:03] tidy error: /checkout/src/libcore/str/mod.rs:31: trailing whitespace
[00:04:05] some tidy checks failed
[00:04:05] 
[00:04:05] 
[00:04:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:05] 
[00:04:05] 
[00:04:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:05] Build completed unsuccessfully in 0:00:48
[00:04:05] Build completed unsuccessfully in 0:00:48
[00:04:05] make: *** [tidy] Error 1
[00:04:05] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:039f7000
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 10 20:46:25 UTC 2019
---
travis_time:end:0116c897:start=1547153186368427731,finish=1547153186373485481,duration=5057750
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c384800
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05f1e94c
travis_time:start:05f1e94c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b96ed00
$ dmesg | grep -i kill

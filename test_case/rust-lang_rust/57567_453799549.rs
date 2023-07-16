plain
travis_time:end:0016d983:start=1547350826893746304,finish=1547350827803811255,duration=910064951
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:tidy
tidy check
[00:03:59] * 569 error codes
[00:03:59] * highest error code: E0721
[00:03:59] tidy error: /checkout/src/libcore/option.rs:1024: malformed stability attribute
[00:03:59] tidy error: /checkout/src/libcore/result.rs:983: malformed stability attribute
[00:04:00] some tidy checks failed
[00:04:00] 
[00:04:00] 
[00:04:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:00] 
[00:04:00] 
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:00] Build completed unsuccessfully in 0:00:48
[00:04:00] Build completed unsuccessfully in 0:00:48
[00:04:00] make: *** [tidy] Error 1
[00:04:00] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:031834da
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 03:44:39 UTC 2019
---
travis_time:end:0dac1fa1:start=1547351080154265745,finish=1547351080158992100,duration=4726355
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00fec0e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b00c403
travis_time:start:0b00c403
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13131c4b
$ dmesg | grep -i kill

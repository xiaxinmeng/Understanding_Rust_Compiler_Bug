plain
travis_time:end:0f50c84d:start=1545948501937726445,finish=1545948582472236327,duration=80534509882
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:04] tidy error: /checkout/src/libpanic_abort/lib.rs:55: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:20: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:22: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:34: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:70: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:74: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:77: line longer than 100 chars
[00:03:05] tidy error: /checkout/src/libstd/sys/wasm_cloudabi/err.rs:92: line longer than 100 chars
[00:03:06] some tidy checks failed
[00:03:06] 
[00:03:06] 
[00:03:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:06] 
[00:03:06] 
[00:03:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:06] Build completed unsuccessfully in 0:00:43
[00:03:06] Build completed unsuccessfully in 0:00:43
[00:03:06] make: *** [tidy] Error 1
[00:03:06] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0751bc50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 27 22:12:57 UTC 2018
---
travis_time:end:04d339a0:start=1545948777942180002,finish=1545948777946553826,duration=4373824
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03b7fc27
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a0fffae
travis_time:start:2a0fffae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1720062b
$ dmesg | grep -i kill

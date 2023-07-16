plain
travis_time:end:0010ef1c:start=1545522018201510058,finish=1545522019324220004,duration=1122709946
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:53] tidy error: /checkout/src/test/ui/fmt/format-string-error-2.rs:41: trailing whitespace
[00:02:55] some tidy checks failed
[00:02:55] 
[00:02:55] 
[00:02:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:55] 
[00:02:55] 
[00:02:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:55] Build completed unsuccessfully in 0:00:46
[00:02:55] Build completed unsuccessfully in 0:00:46
[00:02:55] make: *** [tidy] Error 1
[00:02:55] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05e7b582
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 22 23:43:22 UTC 2018
---
travis_time:end:26ae9acc:start=1545522203314739431,finish=1545522203319259019,duration=4519588
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:316d2eec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:083a8aec
travis_time:start:083a8aec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d53a4b0
$ dmesg | grep -i kill

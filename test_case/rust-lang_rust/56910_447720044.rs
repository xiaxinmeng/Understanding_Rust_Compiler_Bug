plain
travis_time:end:0095eb81:start=1545020603513095240,finish=1545020672499601374,duration=68986506134
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:02:58] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:59] tidy error: /checkout/src/libsyntax/parse/lexer/tokentrees.rs:105: line longer than 100 chars
[00:03:00] some tidy checks failed
[00:03:00] 
[00:03:00] 
[00:03:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:00] 
[00:03:00] 
[00:03:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:00] Build completed unsuccessfully in 0:00:45
[00:03:00] Build completed unsuccessfully in 0:00:45
[00:03:00] Makefile:79: recipe for target 'tidy' failed
[00:03:00] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10a3a204
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 04:27:41 UTC 2018
---
travis_time:end:03367800:start=1545020862205388773,finish=1545020862210753046,duration=5364273
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0294357f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bd1d4f0
travis_time:start:0bd1d4f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fb02ef8
$ dmesg | grep -i kill

plain
travis_time:end:04dc5a70:start=1555987282515188952,finish=1555987367135610049,duration=84620421097
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:27] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:27] tidy error: /checkout/src/libsyntax/parse/parser.rs:3599: trailing whitespace
[00:03:27] tidy error: /checkout/src/test/ui/parser/expr-as-stmt.rs: missing trailing newline
[00:03:28] some tidy checks failed
[00:03:28] 
[00:03:28] 
[00:03:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:28] 
[00:03:28] 
[00:03:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:28] Build completed unsuccessfully in 0:00:42
[00:03:28] Build completed unsuccessfully in 0:00:42
[00:03:28] Makefile:67: recipe for target 'tidy' failed
[00:03:28] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1331d248
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 02:46:25 UTC 2019
---
travis_time:end:11d4395f:start=1555987585997563612,finish=1555987586001891606,duration=4327994
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0205b840
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22d0d600
travis_time:start:22d0d600
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b86ef28
$ dmesg | grep -i kill

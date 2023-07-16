plain
travis_time:end:047249a5:start=1549360058539345807,finish=1549360060908042248,duration=2368696441
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:15] tidy error: /checkout/src/libsyntax/parse/parser.rs: missing trailing newline
[00:04:17] some tidy checks failed
[00:04:17] 
[00:04:17] 
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] 
[00:04:17] 
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:49
[00:04:17] Build completed unsuccessfully in 0:00:49
[00:04:17] Makefile:68: recipe for target 'tidy' failed
[00:04:17] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e8b18cf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 09:52:08 UTC 2019
---
travis_time:end:29f9efdc:start=1549360329540843109,finish=1549360329546469955,duration=5626846
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dd70849
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20fb8dd7
travis_time:start:20fb8dd7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0258ae5c
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[    1.134257] misc rfkill: hash matches
travis_fold:end:after_failure.6

Done. Your build exited with 1.

plain
travis_time:end:0f089337:start=1547502980065685847,finish=1547503093718173008,duration=113652487161
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:55] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:55] tidy error: /checkout/src/libcore/slice/mod.rs:2383: trailing whitespace
[00:03:55] tidy error: /checkout/src/libcore/slice/mod.rs:2384: line longer than 100 chars
[00:03:55] tidy error: /checkout/src/test/ui/slice/slice-idx.rs:1: trailing whitespace
[00:03:55] tidy error: /checkout/src/test/ui/slice/slice-mut-idx.rs:3: trailing whitespace
[00:03:57] some tidy checks failed
[00:03:57] 
[00:03:57] 
[00:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:57] 
[00:03:57] 
[00:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:57] Build completed unsuccessfully in 0:00:52
[00:03:57] Build completed unsuccessfully in 0:00:52
[00:03:57] make: *** [tidy] Error 1
[00:03:57] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04278efd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 14 22:02:19 UTC 2019
---
travis_time:end:0d5b7a32:start=1547503340691036326,finish=1547503340695740254,duration=4703928
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08e67398
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03f6ba24
travis_time:start:03f6ba24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08272f4c
$ dmesg | grep -i kill

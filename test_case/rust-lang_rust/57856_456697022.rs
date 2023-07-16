plain
travis_time:end:09559c14:start=1548227820661750598,finish=1548227822611622372,duration=1949871774
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:44] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:44] tidy error: /checkout/src/libproc_macro/lib.rs:8: line longer than 100 chars
[00:03:44] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:1546: line longer than 100 chars
[00:03:44] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:2921: line longer than 100 chars
[00:03:44] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:3027: line longer than 100 chars
[00:03:44] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:3036: line longer than 100 chars
[00:03:44] tidy error: /checkout/src/librustc/diagnostics.rs:645: line longer than 100 chars
[00:03:45] some tidy checks failed
[00:03:45] 
[00:03:45] 
[00:03:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:45] 
[00:03:45] 
[00:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:45] Build completed unsuccessfully in 0:00:47
[00:03:45] Build completed unsuccessfully in 0:00:47
[00:03:45] Makefile:69: recipe for target 'tidy' failed
[00:03:45] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02f2a1ab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 23 07:21:00 UTC 2019
---
travis_time:end:146efc9c:start=1548228060608080289,finish=1548228060612906734,duration=4826445
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10bd70c4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0574dae0
travis_time:start:0574dae0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06895c38
$ dmesg | grep -i kill

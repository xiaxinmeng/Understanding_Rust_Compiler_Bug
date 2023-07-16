plain
travis_time:end:036bc6a0:start=1544115320588734196,finish=1544115322713412186,duration=2124677990
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
tidy check
[00:03:45] * 568 error codes
[00:03:45] * highest error code: E0721
[00:03:46] * 242 features
[00:03:46] invalid source: "git+https://github.com/rust-lang-nursery/rust-clippy?rev=f5d868c9edfc6c2a9310d564a2f738bec67dfd6b#f5d868c9edfc6c2a9310d564a2f738bec67dfd6b"
[00:03:46] invalid source: "git+https://github.com/rust-lang-nursery/rust-clippy?rev=f5d868c9edfc6c2a9310d564a2f738bec67dfd6b#f5d868c9edfc6c2a9310d564a2f738bec67dfd6b"
[00:03:46] some tidy checks failed
[00:03:46] 
[00:03:46] 
[00:03:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:46] 
[00:03:46] 
[00:03:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:46] Build completed unsuccessfully in 0:00:57
[00:03:46] Build completed unsuccessfully in 0:00:57
[00:03:46] Makefile:79: recipe for target 'tidy' failed
[00:03:46] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0456b30a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec  6 16:59:20 UTC 2018
---
travis_time:end:0f1e1310:start=1544115561114356868,finish=1544115561120312768,duration=5955900
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11c91c9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003890d0
travis_time:start:003890d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05d53320
$ dmesg | grep -i kill

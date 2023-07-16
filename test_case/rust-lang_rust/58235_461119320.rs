plain
travis_time:end:102dd250:start=1549472906374749143,finish=1549473004216121198,duration=97841372055
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:07:28] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:07:28] tidy error: /checkout/src/libstd/sys/sgx/abi/usercalls/raw.rs:26: line longer than 100 chars
[00:07:28] tidy error: /checkout/src/libstd/sys/sgx/abi/usercalls/raw.rs:173: line longer than 100 chars
[00:07:28] tidy error: /checkout/src/libstd/sys/sgx/abi/usercalls/raw.rs:189: line longer than 100 chars
[00:07:28] tidy error: /checkout/src/libstd/sys/sgx/abi/usercalls/raw.rs:205: line longer than 100 chars
[00:07:28] tidy error: /checkout/src/libstd/sys/sgx/abi/usercalls/raw.rs:220: line longer than 100 chars
[00:07:28] tidy error: /checkout/src/libstd/sys/sgx/abi/usercalls/raw.rs:234: line longer than 100 chars
[00:07:29] some tidy checks failed
[00:07:29] 
[00:07:29] 
[00:07:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:07:29] 
[00:07:29] 
[00:07:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:07:29] Build completed unsuccessfully in 0:00:46
[00:07:29] Build completed unsuccessfully in 0:00:46
[00:07:29] make: *** [tidy] Error 1
[00:07:29] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00473384
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 17:17:52 UTC 2019
---
travis_time:end:0e88b464:start=1549473473520725912,finish=1549473473525365568,duration=4639656
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03d2cafd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0566f070
travis_time:start:0566f070
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3b874afc
$ dmesg | grep -i kill

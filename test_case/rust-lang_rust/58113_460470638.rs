plain
travis_time:end:01f50d83:start=1549327025090644957,finish=1549327124371093622,duration=99280448665
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
####################################################################      94.6%
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:58]     Updating crates.io index
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:30
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:10] Command failed. Attempt 2/5:
[00:02:10] Command failed. Attempt 2/5:
[00:02:11] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:11] Build completed unsuccessfully in 0:00:00
[00:02:11] make: *** [prepare] Error 1
[00:02:11] Makefile:70: recipe for target 'prepare' failed
[00:02:13] Command failed. Attempt 3/5:
[00:02:13] Command failed. Attempt 3/5:
[00:02:13] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:13] Build completed unsuccessfully in 0:00:00
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:70: recipe for target 'prepare' failed
[00:02:16] Command failed. Attempt 4/5:
[00:02:16] Command failed. Attempt 4/5:
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:70: recipe for target 'prepare' failed
[00:02:20] Command failed. Attempt 5/5:
[00:02:20] Command failed. Attempt 5/5:
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] Makefile:70: recipe for target 'prepare' failed
[00:02:21] The command has failed after 5 attempts.
[00:02:21] make: *** [prepare] Error 1
---
travis_time:end:0eea3300:start=1549327276648467364,finish=1549327276655192721,duration=6725357
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01178f9e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0442741c
travis_time:start:0442741c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05dcf59a
$ dmesg | grep -i kill

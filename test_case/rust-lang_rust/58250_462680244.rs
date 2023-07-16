plain
travis_time:end:1073c4b2:start=1549962831646985852,finish=1549962834386003846,duration=2739017994
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:27] 
######################################################################## 100.0%
[00:02:28] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:28]     Updating crates.io index
[00:02:45] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:45] Build completed unsuccessfully in 0:00:59
[00:02:45] Makefile:70: recipe for target 'prepare' failed
[00:02:45] make: *** [prepare] Error 1
[00:02:46] Command failed. Attempt 2/5:
[00:02:46] Command failed. Attempt 2/5:
[00:02:46] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:46] Build completed unsuccessfully in 0:00:00
[00:02:46] make: *** [prepare] Error 1
[00:02:46] Makefile:70: recipe for target 'prepare' failed
[00:02:48] Command failed. Attempt 3/5:
[00:02:48] Command failed. Attempt 3/5:
[00:02:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:48] Build completed unsuccessfully in 0:00:00
[00:02:48] Makefile:70: recipe for target 'prepare' failed
[00:02:48] make: *** [prepare] Error 1
[00:02:51] Command failed. Attempt 4/5:
[00:02:51] Command failed. Attempt 4/5:
[00:02:52] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:52] Build completed unsuccessfully in 0:00:00
[00:02:52] make: *** [prepare] Error 1
[00:02:52] Makefile:70: recipe for target 'prepare' failed
[00:02:56] Command failed. Attempt 5/5:
[00:02:56] Command failed. Attempt 5/5:
[00:02:56] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:56] Build completed unsuccessfully in 0:00:00
[00:02:56] Makefile:70: recipe for target 'prepare' failed
[00:02:56] make: *** [prepare] Error 1
[00:02:56] The command has failed after 5 attempts.
---
travis_time:end:0a0df532:start=1549963023338060582,finish=1549963023342592626,duration=4532044
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:057d5054
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d7e4641
travis_time:start:2d7e4641
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02f553a3
$ dmesg | grep -i kill

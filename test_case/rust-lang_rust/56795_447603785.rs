plain
travis_time:end:0124f144:start=1544914411276497938,finish=1544914466154306241,duration=54877808303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
#####################################################################     97.2%
######################################################################## 100.0%
[00:01:29] extracting /checkout/obj/build/cache/2018-12-09/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:30]     Updating crates.io index
[00:01:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:37] Build completed unsuccessfully in 0:00:30
[00:01:37] make: *** [prepare] Error 1
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:38] Command failed. Attempt 2/5:
[00:01:38] Command failed. Attempt 2/5:
[00:01:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:38] Build completed unsuccessfully in 0:00:00
[00:01:38] make: *** [prepare] Error 1
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:40] Command failed. Attempt 3/5:
[00:01:40] Command failed. Attempt 3/5:
[00:01:40] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:40] Build completed unsuccessfully in 0:00:00
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:40] make: *** [prepare] Error 1
[00:01:43] Command failed. Attempt 4/5:
[00:01:43] Command failed. Attempt 4/5:
[00:01:44] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:44] Build completed unsuccessfully in 0:00:00
[00:01:44] make: *** [prepare] Error 1
[00:01:44] Makefile:81: recipe for target 'prepare' failed
[00:01:48] Command failed. Attempt 5/5:
[00:01:48] Command failed. Attempt 5/5:
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] Makefile:81: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:48] The command has failed after 5 attempts.
---
travis_time:end:163733fc:start=1544914583783216249,finish=1544914583790336040,duration=7119791
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d0feaf1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:120a27cb
travis_time:start:120a27cb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:074ca48c
$ dmesg | grep -i kill

plain
travis_time:end:0104e220:start=1548596503910606575,finish=1548596504830547663,duration=919941088
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:31] 
######################################################################## 100.0%
[00:01:32] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:32]     Updating crates.io index
[00:01:42] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:42] Build completed unsuccessfully in 0:00:25
[00:01:42] make: *** [prepare] Error 1
[00:01:42] Makefile:70: recipe for target 'prepare' failed
[00:01:43] Command failed. Attempt 2/5:
[00:01:43] Command failed. Attempt 2/5:
[00:01:43]     Updating crates.io index
[00:01:44] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:44] Build completed unsuccessfully in 0:00:00
[00:01:44] make: *** [prepare] Error 1
[00:01:44] Makefile:70: recipe for target 'prepare' failed
[00:01:46] Command failed. Attempt 3/5:
[00:01:46] Command failed. Attempt 3/5:
[00:01:46]     Updating crates.io index
[00:01:46] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:46] Build completed unsuccessfully in 0:00:00
[00:01:46] make: *** [prepare] Error 1
[00:01:46] Makefile:70: recipe for target 'prepare' failed
[00:01:49] Command failed. Attempt 4/5:
[00:01:49] Command failed. Attempt 4/5:
[00:01:49]     Updating crates.io index
[00:01:50] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:50] Build completed unsuccessfully in 0:00:00
[00:01:50] Makefile:70: recipe for target 'prepare' failed
[00:01:50] make: *** [prepare] Error 1
[00:01:54] Command failed. Attempt 5/5:
[00:01:54] Command failed. Attempt 5/5:
[00:01:54]     Updating crates.io index
[00:01:55] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] Makefile:70: recipe for target 'prepare' failed
[00:01:55] make: *** [prepare] Error 1
[00:01:55] The command has failed after 5 attempts.
---
travis_time:end:1e8f6b8c:start=1548596635015195825,finish=1548596635022658102,duration=7462277
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1efd3954
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0490bdcb
travis_time:start:0490bdcb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:24bf1cc8
$ dmesg | grep -i kill

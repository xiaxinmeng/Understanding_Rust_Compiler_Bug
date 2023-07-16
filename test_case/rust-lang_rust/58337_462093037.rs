plain
travis_time:end:333e754c:start=1549759091355204723,finish=1549759093484564248,duration=2129359525
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
####################################################################      94.8%
######################################################################## 100.0%
[00:02:10] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:10]     Updating crates.io index
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:34
[00:02:21] Makefile:70: recipe for target 'prepare' failed
[00:02:21] make: *** [prepare] Error 1
[00:02:22] Command failed. Attempt 2/5:
[00:02:22] Command failed. Attempt 2/5:
[00:02:22]     Updating crates.io index
[00:02:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:70: recipe for target 'prepare' failed
[00:02:24] Command failed. Attempt 3/5:
[00:02:24] Command failed. Attempt 3/5:
[00:02:25]     Updating crates.io index
[00:02:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] Makefile:70: recipe for target 'prepare' failed
[00:02:25] make: *** [prepare] Error 1
[00:02:28] Command failed. Attempt 4/5:
[00:02:28] Command failed. Attempt 4/5:
[00:02:28]     Updating crates.io index
[00:02:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] make: *** [prepare] Error 1
[00:02:29] Makefile:70: recipe for target 'prepare' failed
[00:02:33] Command failed. Attempt 5/5:
[00:02:33] Command failed. Attempt 5/5:
[00:02:33]     Updating crates.io index
[00:02:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:33] Build completed unsuccessfully in 0:00:00
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:70: recipe for target 'prepare' failed
[00:02:33] The command has failed after 5 attempts.
---
travis_time:end:003c43b3:start=1549759260691181591,finish=1549759260695901553,duration=4719962
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:113ca358
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0de1816b
travis_time:start:0de1816b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00c2e4c1
$ dmesg | grep -i kill
